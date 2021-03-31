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
