mod exports {
    #[cfg(feature = "atari")]
    pub use atari_env as atari;
    // #[cfg(feature = "openspiel")]
    // pub use openspiel_env as openspiel;
}
pub use exports::*;

#[cfg(feature = "atari")]
use atari_env::{AtariEnv, AtariRamEnv, AtariRgbEnv};

/// Gym is a collection of known envs.
///
/// Example:
/// let gym = GymRegistry::new();
/// let envs = vec![
///   gym::continuous_env("atari-breakout-v0"),
///   gym::continuous_env("atari-pong-v0"),
/// ];
use anyhow::{Context, Result};
use gym_core::{ContinuousEnv, DiscreteEnv};

pub struct Gym {
    continuous_envs: std::collections::HashMap<String, Box<dyn Fn() -> ContinuousEnv>>,
    discrete_envs: std::collections::HashMap<String, Box<dyn Fn() -> DiscreteEnv>>,
}

impl Gym {
    pub fn new() -> Result<Self> {
        let mut s: Self = Self {
            continuous_envs: std::collections::HashMap::new(),
            discrete_envs: std::collections::HashMap::new(),
        };

        #[cfg(feature = "atari")]
        if let Ok(dir) = std::env::var("ATARI_ROMS_DIR") {
            let dir = std::path::PathBuf::from(dir);
            if dir.is_dir() {
                for entry in dir.read_dir()? {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        let name: &str = path
                            .file_stem()
                            .with_context(|| format!("filename error: {}", entry.path().display()))?
                            .to_str()
                            .context("atari rom filename is not utf-8")?;
                        let name = name.replace("_", "-");
                        {
                            let path = path.clone();
                            s.discrete_envs.insert(
                                format!("atari-{}-ram", name),
                                Box::new(move || {
                                    Box::new(AtariRamEnv::new(AtariEnv::new(
                                        path.clone(),
                                        Default::default(),
                                    )))
                                }),
                            );
                        }
                        {
                            let path = path.clone();
                            s.discrete_envs.insert(
                                format!("atari-{}-rgb", name),
                                Box::new(move || {
                                    Box::new(AtariRgbEnv::new(AtariEnv::new(
                                        path.clone(),
                                        Default::default(),
                                    )))
                                }),
                            );
                        }
                    }
                }
            } else {
                anyhow::bail!("ATARI_ROMS_DIR({}) is not a directory", dir.display());
            }
        } else {
            anyhow::bail!("Please set ATARI_ROMS_DIR ( e.g. export ATARI_ROMS_DIR=~/.local/lib/python3.9/site-packages/atari_py/atari_roms/ )")
        }

        Ok(s)
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
    pub fn discrete_env(&self, name: &str) -> Option<DiscreteEnv> {
        self.discrete_envs.get(name).map(|f| f())
    }
}
