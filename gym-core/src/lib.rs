use anyhow::Result;
// - GymEnv<ObservationSpace=Observation2D, ActionSpace=DiscreteAction2D>
// - impl GymEnv<Observation=[u8], Action=AtariAction> for AtariEnv
// - impl<O,A> GymEnv<Observation=ndarray::Array<O>, Action=ndarray::Array<A>> for DynamicEnv<O,A>

// GymEnv<ObservationSpace>=

// impl GymEnv<Observation=i8, DiscreteAction> for AtariEnv


// GymEnv provides a generic interface to various environments.
pub trait GymEnv<Action> {
    fn state(&self) -> ndarray::ArrayView<f32, ndarray::IxDyn>;
    fn step(&mut self, action: Action) -> Result<i32>;
    // whether the episode is over
    fn is_over(&self) -> bool;
    // resets the episode
    fn reset(&mut self);
}

pub type ContinuousEnv = Box<dyn GymEnv<ndarray::ArrayD<f32>>>;
pub type DiscreteEnv = Box<dyn GymEnv<ndarray::ArrayD<i32>>>;

// Examples:
// let mut envs: Vec<GeneralEnv> = vec![];
// envs.push(AtariEnv::new("breakout-v1.out").as_general_env(ObservationType::Rgb24));
// envs.push(AtariEnv::new("tetris-v1.out").as_general_env());
// envs.push(CartPole::new().as_general_env(ActionType::Continuous));
// // same experiment code for all envs
