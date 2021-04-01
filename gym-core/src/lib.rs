use anyhow::Result;
mod action_space;
pub use action_space::{
    ActionSpace, CategoricalAction, ContinuousAction, MultiCategoricalAction, MultiContinuousAction,
};

// GymEnv provides a generic interface to various environments.
pub trait GymEnv<ActionDtype> {
    // a scalar is considered 0-dim (empty Vec)
    fn state_size(&self) -> Vec<usize>;
    // a scalar is considered 0-dim (empty Vec)
    fn action_size(&self) -> Vec<usize>;

    /// it's hard to cover all types of action_space:
    /// discrete/continuous, box, hierarchical, etc
    /// Supported action types:
    /// - Single Discrete Integer (n)
    /// - N-dim discrete integers (n, num_action for each dim)
    /// - Single Continuous (low, high)
    /// - N-dim continuous (shape, n-dim low, n-dim high)
    fn action_space(&self) -> ActionSpace<ActionDtype>;

    fn state(&self, out: ndarray::ArrayViewMut<f32, ndarray::IxDyn>) -> Result<()>;
    fn step(&mut self, action: ndarray::ArrayD<ActionDtype>) -> Result<i32>;
    // whether the episode is over
    fn is_over(&self) -> bool;
    // resets the episode
    fn reset(&mut self);
}

pub type ContinuousEnv = Box<dyn GymEnv<f32>>;
pub type DiscreteEnv = Box<dyn GymEnv<i32>>>;

// Examples:
// let mut envs: Vec<GeneralEnv> = vec![];
// envs.push(AtariEnv::new("breakout-v1.out").as_general_env(ObservationType::Rgb24));
// envs.push(AtariEnv::new("tetris-v1.out").as_general_env());
// envs.push(CartPole::new().as_general_env(ActionType::Continuous));
// // same experiment code for all envs
