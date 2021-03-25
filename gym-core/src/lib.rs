
/// GymEnv provides a general interface to various environments.
trait GymEnv {
    type ObservationSpace = ;
    type ActionSpace = ;
    
    fn observation_space(&self) -> Vec<usize>;
    fn action_space(&self) -> Vec<usize>;
    fn state(&self, ) -> ndarray::ArrayView<f64>;
    fn step(&self) -> i32;
    fn is_over(&self) -> bool;
    fn reset(&self);
}

/// Examples:
/// let mut envs: Vec<GeneralEnv> = vec![];
/// envs.push(AtariEnv::new("breakout-v1.out").as_general_env(ObservationType::Rgb24));
/// envs.push(AtariEnv::new("tetris-v1.out").as_general_env());
/// envs.push(CartPole::new().as_general_env(ActionType::Continuous));
/// // same experiment code for all envs
/// train(envs[0]);
/// train(envs[1]);
/// train(envs[2]);
type GeneralEnv = Box<dyn GymEnv>;
