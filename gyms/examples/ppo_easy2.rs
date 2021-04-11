use tch::nn;
use tch::nn::OptimizerConfig;
use tch::Tensor;
use colored::Colorize;
use inline_python::{Context, python};

// architecture borrowed from https://github.com/LaurentMazare/tch-rs/blob/master/examples/reinforcement-learning/ppo.rs
fn new_network(p: &nn::Path, n: usize, n_actions: usize) -> Box<dyn Fn(&Tensor) -> (Tensor, Tensor)> {
    let seq = nn::seq()
        .add_fn(|xs| xs.flat_view())
        // .add(nn::linear(p / "l1", 960, 512, Default::default()))
        .add(nn::linear(p / "l1", (n*n+1) as i64, 32, Default::default()))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(p / "l2", 32, 16, Default::default()))
        .add_fn(|xs| xs.relu())
        ;
        
    let critic = nn::linear(p / "cl", 16, 1, Default::default());
    let actor = nn::seq()
        .add(nn::linear(
            p / "al",
            16,
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
    action: usize,
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

pub fn debug_print_probs(n: usize, probs: Tensor, action: Option<usize>) {
    println!("action:");
    let y = action.map(|a| a / n).unwrap_or(n);
    let x = action.map(|a| a % n).unwrap_or(n);
    for i in 0..n {
        for j in 0..n {
            let value: u8 = (probs.double_value(&[(i*n+j) as i64]) as f32  * 255.0) as u8;
            if i==y && j==x {
                print!("{}", "O".on_truecolor(value,value, value).truecolor(255-value, 255-value, 0));
            } else {
                print!("{}", " ".on_truecolor(value,value, value));
            }
            
        }
        println!("");
    }
}

fn main() {
    color_backtrace::install();
    let c: Context = python! {
        import wandb
        wandb.init(project="ppo")
    };

    let n = 5;
    let mut env = gyms::easy_env::EasyEnv2::new(n);

    let device = tch::Device::Cpu;
    let vs = nn::VarStore::new(device);
    let net = new_network(&vs.root(), env.n, env.nn);
    let mut opt = nn::Adam::default().build(&vs, 1e-4).unwrap();

    // params
    let epsilon = 0.1;
    let batchsize = 128;
    let episode_max_step = 1000;

    let mut buf = Vec::<u8>::new();
    buf.resize(env.state_size(), 0);

    let mut data = TrainSet::new(episode_max_step, &[env.state_size()], device);

    let mut score = 0;
    // episode_progress.set_message("Episode");
    for _episode in 0..10000 {
        // PPO is on-policy.
        data.reset();

        let mut episode_score = 0;

        for _step in 0..episode_max_step {

            let state: Tensor = Tensor::from(env.state()).to_kind(tch::Kind::Float).set_requires_grad(false) / 255.0;
            let state = state
                .reshape(&[1, env.state_size() as i64]);
            let (values, probs) = tch::no_grad(|| net(&state)); // [BATCH, 1], [BATCH, N_ACTION]
            let action: usize = probs.get(0).multinomial(1, true).squeeze1(0).int64_value(&[]) as usize;
            let reward = env.step(action);
            // println!("reward {}", reward);
            score += reward;
            data.push(TrainSample {
                state: state.get(0).get(0),
                action,
                value: values.squeeze(),
                reward,
                prob_of_action: probs.squeeze1(0).max().double_value(&[]) as f32,
                is_over: env.is_over(),
            });

            episode_score += reward;

            if _step==episode_max_step-1 {
                println!("episode {} step {} reward {} score {} action {} - avg score {}", _episode,_step, reward, score, action, episode_score as f64 / (_step+1) as f64);
                c.run(python! {
                    wandb.log({"episode_score": 'episode_score})
                });
                env.debug_print_state(Some(action));
                debug_print_probs(env.n, probs.get(0), Some(action));
            }

            if env.is_over () {
                score = 0;
                env.reset();
            }
        }

        let last_q_s: Tensor =
            tch::no_grad(|| net(&data.states.get(data.len as i64 - 1).unsqueeze(0)).0);
        data.finish(last_q_s.squeeze());

        // update policy
        for _update_step in 0..1000 {
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
