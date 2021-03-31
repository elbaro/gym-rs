// - GymEnv<ObservationSpace=Observation2D, ActionSpace=DiscreteAction2D>
// - impl GymEnv<Observation=[u8], Action=AtariAction> for AtariEnv
// - impl<O,A> GymEnv<Observation=ndarray::Array<O>, Action=ndarray::Array<A>> for DynamicEnv<O,A>

// GymEnv<ObservationSpace>=

// impl GymEnv<Observation=i8, DiscreteAction> for AtariEnv


// GymEnv provides a generic interface to various environments.
pub trait GymEnv<Observation: ?Sized, Action> {
    fn state(&self, observation: &mut Observation);
    fn step(&mut self, action: Action) -> Result<i32>;
    // whether the episode is over
    fn is_over(&self) -> bool;
    // resets the episode
    fn reset(&mut self);
}

// example:
// impl GymEnv<[u8;W*H*3], AtariAction> for AtariEnv
// impl GymEnv<ArrayView, Array> for AtariEnv


/// ContinuousEnv and DiscreteEnv provide a general interface for envs.
/// i32 observations are converted to f32.
pub type ContinuousEnv<'a> = Box<dyn GymEnv<ndarray::ArrayViewMut<'a, f32, ndarray::IxDyn>, ndarray::ArrayD<f32>>>;
pub type DiscreteEnv<'a> = Box<dyn GymEnv<ndarray::ArrayViewMut<'a, f32, ndarray::IxDyn>, ndarray::ArrayD<i32>>>;

// Examples:
// let mut envs: Vec<GeneralEnv> = vec![];
// envs.push(AtariEnv::new("breakout-v1.out").as_general_env(ObservationType::Rgb24));
// envs.push(AtariEnv::new("tetris-v1.out").as_general_env());
// envs.push(CartPole::new().as_general_env(ActionType::Continuous));
// // same experiment code for all envs
// train(envs[0]);
// train(envs[1]);
// train(envs[2]);

