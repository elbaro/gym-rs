use ::num::FromPrimitive;
use atari_env::{AtariAction, AtariEnv, EmulatorConfig};
use indicatif::ProgressBar;
use tch::nn;
use tch::nn::OptimizerConfig;
use tch::Tensor;
use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(default_value="/home/emppu/.local/lib/python3.9/site-packages/atari_py/atari_roms/zaxxon.bin")]
    rom: std::path::PathBuf,
}

// architecture borrowed from https://github.com/LaurentMazare/tch-rs/blob/master/examples/reinforcement-learning/ppo.rs
fn new_network(p: &nn::Path, n_actions: usize) -> Box<dyn Fn(&Tensor) -> (Tensor, Tensor)> {
    let stride = |s| nn::ConvConfig {
        stride: s,
        ..Default::default()
    };
    let seq = nn::seq()
        .add(nn::conv2d(p / "c1", 3, 32, 8, stride(4)))
        .add_fn(|xs| xs.relu())
        .add(nn::conv2d(p / "c2", 32, 64, 4, stride(2)))
        .add_fn(|xs| xs.relu())
        .add(nn::conv2d(p / "c3", 64, 64, 4, stride(2)))
        .add_fn(|xs| xs.relu())
        .add(nn::conv2d(p / "c4", 64, 64, 4, stride(2)))
        .add_fn(|xs| xs.relu().flat_view())
        // .add(nn::linear(p / "l1", 960, 512, Default::default()))
        .add(nn::linear(p / "l1", 768, 512, Default::default()))
        .add_fn(|xs| xs.relu());
    let critic = nn::linear(p / "cl", 512, 1, Default::default());
    let actor = nn::seq()
        .add(nn::linear(
            p / "al",
            512,
            n_actions as i64,
            Default::default(),
        ))
        .add_fn(|x| x.softmax(-1, tch::Kind::Float));
    let device = p.device();
    Box::new(move |xs: &Tensor| {
        let xs = xs.to_device(device).apply(&seq);
        (xs.apply(&critic), xs.apply(&actor))
    })
}

struct TrainSample {
    state: Tensor,
    action: AtariAction,
    reward: i32,
    value: Tensor,
    prob_of_action: f32,
    is_over: bool,
}

struct TrainSet {
    len: usize,
    nsteps: usize,
    states: Tensor,
    actions: Tensor,
    rewards: Tensor,
    probs_of_actions: Tensor,
    values: Tensor,
    returns: Tensor,
    are_over: Tensor,
}
impl TrainSet {
    fn new(nsteps: usize, state_shape: &[usize], device: tch::Device) -> Self {
        let shape: Vec<i64> = (&[nsteps + 1])
            .iter()
            .chain(state_shape.iter())
            .map(|x| *x as i64)
            .collect();
        Self {
            len: 0,
            nsteps,
            states: Tensor::zeros(&shape, (tch::Kind::Float, device)),
            actions: Tensor::zeros(&[nsteps as i64], (tch::Kind::Int64, device)),
            rewards: Tensor::zeros(&[nsteps as i64], (tch::Kind::Float, device)),
            probs_of_actions: Tensor::zeros(&[nsteps as i64], (tch::Kind::Float, device)),
            values: Tensor::zeros(&[(nsteps) as i64], (tch::Kind::Float, device)),
            returns: Tensor::zeros(&[nsteps as i64], (tch::Kind::Float, device)),
            are_over: Tensor::zeros(&[nsteps as i64], (tch::Kind::Float, device)),
        }
    }
    #[allow(unused_must_use)]
    fn reset(&mut self) {
        self.len = 0;
        self.states.zero_();
        self.actions.zero_();
        self.rewards.zero_();
        self.probs_of_actions.zero_();
        self.values.zero_();
        self.returns.zero_();
        self.are_over.zero_();
    }
    fn push(&mut self, sample: TrainSample) {
        if self.len == self.nsteps {
            panic!("TrainSet overflow");
        }
        let device = self.states.device();
        self.states.get(self.len as i64).copy_(&sample.state);
        self.actions
            .get(self.len as i64)
            .copy_(&Tensor::scalar_tensor(
                sample.action as i32 as i64,
                (tch::Kind::Float, device),
            ));
        self.rewards
            .get(self.len as i64)
            .copy_(&Tensor::scalar_tensor(
                sample.reward as i64,
                (tch::Kind::Float, device),
            ));
        self.probs_of_actions
            .get(self.len as i64)
            .copy_(&Tensor::scalar_tensor(
                sample.prob_of_action as f64,
                (tch::Kind::Float, device),
            ));
        self.values.get(self.len as i64).copy_(&sample.value);
        self.are_over
            .get(self.len as i64)
            .copy_(&Tensor::scalar_tensor(
                sample.is_over as i32 as f64,
                (tch::Kind::Float, device),
            ));
        self.len += 1;
    }
    fn finish(&mut self, last_q_s: Tensor) {
        // fill in advantages
        // A(s,a) = R(s,a) - Q(s)

        let returns = Tensor::zeros(&[(self.len + 1) as i64], tch::kind::FLOAT_CPU);
        returns.get(-1).copy_(&last_q_s); // Q(s_gameover)

        for i in (0..self.len as i64).rev() {
            let mask = Tensor::from(1_f32) - self.are_over.get(i);
            let return_: Tensor = self.rewards.get(i) + returns.get(i + 1) * mask * 0.99;
            returns.get(i).copy_(&return_);
        }
        // nsteps+1 => nsteps
        self.returns = returns.narrow(0, 0, self.len as i64);
    }
    fn sample(&self, n: usize) -> Self {
        let indices = Tensor::randint(self.len as i64, &[n as i64], tch::kind::INT64_CPU);
        Self {
            len: n,
            nsteps: 0,
            states: self.states.index_select(0, &indices),
            actions: self.actions.index_select(0, &indices),
            rewards: self.rewards.index_select(0, &indices),
            probs_of_actions: self.probs_of_actions.index_select(0, &indices),
            values: self.values.index_select(0, &indices),
            returns: self.returns.index_select(0, &indices),
            are_over: self.are_over.index_select(0, &indices),
        }
    }
}

fn main() {
    color_backtrace::install();
    let opts = Opts::parse();

    let mut env = AtariEnv::new(
        opts.rom,
        // "/home/emppu/.local/lib/python3.9/site-packages/atari_py/atari_roms/carnival.bin",
        EmulatorConfig {
            display_screen: true,
            ..EmulatorConfig::default()
        },
    );
    let mut env = gyms::easy_env::EasyEnv2::new(4);

    let device = tch::Device::Cpu;
    let vs = nn::VarStore::new(device);
    let net = new_network(&vs.root(), env.available_actions().len());
    let mut opt = nn::Adam::default().build(&vs, 1e-4).unwrap();

    // params
    let epsilon = 0.1;
    let batchsize = 32;
    let episode_max_step = 1000;

    let mut buf = Vec::<u8>::new();
    let mut buf2 = Vec::<f32>::new();
    buf.resize(env.rgb24_size(), 0);
    buf2.resize(env.rgb24_size(), 0.0);

    let mut data = TrainSet::new(episode_max_step, &[3, env.height(), env.width()], device);
    let height = env.height();
    let width = env.width();

    println!("h {} w {}", height, width);

    let episode_progress = ProgressBar::new(100);
    let mut score = 0;
    let mut lives = env.lives();
    // episode_progress.set_message("Episode");
    for _episode in 0..10000 {
        episode_progress.inc(1);
        // PPO is on-policy.
        data.reset();

        let step_progress = ProgressBar::new(episode_max_step as u64);
        for _step in 0..episode_max_step {
            step_progress.inc(1);
            env.render_rgb24(&mut buf);
            let state: Tensor = Tensor::from(buf.as_slice()).to_kind(tch::Kind::Float).set_requires_grad(false) / 255.0;
            let state = state
                .reshape(&[1, (height * width) as i64, 3_i64])
                .transpose(1, 2)
                .reshape(&[1, 3, height as i64, width as i64]);
            let (values, probs) = tch::no_grad(|| net(&state)); // [BATCH, 1], [BATCH, N_ACTION]
            let action = probs.get(0).multinomial(1, true).squeeze1(0).int64_value(&[]) as i32;
            let action: AtariAction = AtariAction::from_i32(action).unwrap();
            let reward = env.step(action);
            score += reward;
            data.push(TrainSample {
                state: state.get(0),
                action,
                value: values.squeeze(),
                reward,
                prob_of_action: probs.squeeze1(0).max().double_value(&[]) as f32,
                is_over: env.is_game_over(),
            });

            // other rows are filled with 0
            if env.is_game_over()  || env.lives() != lives {
                println!("\nscore {}\n\n", score);
                lives = env.lives();
                score = 0;
                env.reset();
            }
        }

        let last_q_s: Tensor =
            tch::no_grad(|| net(&data.states.get(data.len as i64 - 1).unsqueeze(0)).0);
        data.finish(last_q_s.squeeze());

        // update policy
        for _update_step in 0..100 {
            // https://spinningup.openai.com/en/latest/algorithms/ppo.html
            // sample
            let batch = data.sample(batchsize);
            let (values, policy) = net(&batch.states);
            let advantages = &batch.returns - values.get(0);
            let ratio = policy.index_select(1, &batch.actions) / batch.probs_of_actions;
            let first_term = &ratio * &advantages; // larger
            let second_term = ratio.clip(1.0 - epsilon, 1.0 + epsilon) * &advantages;
            let policy_loss = (-first_term.min1(&second_term)).mean(tch::Kind::Float);
            let value_loss = (&advantages * &advantages).mean(tch::Kind::Float);
            let loss = policy_loss + value_loss;
            opt.backward_step(&loss);
        }
        // update value
    }
}
