mod exports {
    #[cfg(feature = "atari")]
    pub use atari_env as atari;

    #[cfg(feature = "openspiel")]
    pub use openspiel_env as openspiel;
}

pub use exports::*;


/// Gym is a collection of known envs.
///
/// Example:
/// let gym = GymRegistry::new();
/// let envs = vec![
///   gym::continuous_env("atari-breakout-v0"),
///   gym::continuous_env("atari-pong-v0"),
/// ];


struct Gym {
    continuous_envs: std::collections::HashMap<String, fn()->ContinuousEnv>,
    discrete_envs: std::collections::HashMap<String, fn()->DiscreteEnv>,
}

impl Gym {
    pub fn new() -> Self {
        let s : Self = Self { continuous_envs: std::collections::HashMap::new(), discrete_envs: std::collections::HashMap::new() };

        #[cfg(feature="atari")]
        {
            s.continuous_envs.insert("atari-breakout-ram", || AtariRamEnv());
            s.continuous_envs.insert("atari-breakout-rgb", || AtariRgbEnv());
        }

        s
    }
    pub fn continuous_envs(&self) -> Vec<String> {
        self.continuous_envs.keys().cloned().collect()
    }
    pub fn discrete_envs(&self) -> Vec<String> {
        self.discrete_envs.keys().cloned().collect()
    }

    pub fn continuous_env(&self, name: &str) -> Option<ContinuousEnv> {
        self.continuous_envs.get(name).map(|f| f())
    }
    pub fn discrete_env(&self, name:&str) -> Option<DiscreteEnv> {
        self.discrete_envs.get(name).map(|f| f())
    }
}
