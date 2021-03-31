pub mod ale;
use std::path::Path;

pub use ale::Ale;
pub use ale::AleAction as AtariAction;
pub use ale::AleConfig as EmulatorConfig;

use gym_core::GymEnv;

pub struct AtariEnv {
    ale: Ale,
}

impl AtariEnv {
    /// about frame-skipping and action-repeat,
    /// see <https://danieltakeshi.github.io/2016/11/25/frame-skipping-and-preprocessing-for-deep-q-networks-on-atari-2600-games/>
    pub fn new<P: AsRef<Path>>(rom_path: P, emulator_config: EmulatorConfig) -> Self {
        Self {
            ale: Ale::new(rom_path.as_ref(), emulator_config),
        }
    }

    pub fn width(&self) -> u32 {
        self.ale.width()
    }
    pub fn height(&self) -> u32 {
        self.ale.height()
    }
    pub fn available_actions(&self) -> Vec<AtariAction> {
        self.ale.available_actions()
    }
    pub fn minimal_actions(&self) -> Vec<AtariAction> {
        self.ale.minimal_actions()
    }
    pub fn is_game_over(&self) -> bool {
        self.ale.is_game_over()
    }

    pub fn reset(&mut self) {
        self.ale.reset()
    }
    pub fn step(&mut self, action: AtariAction) -> i32 {
        self.ale.take_action(action)
    }

    pub fn rgb32_size(&self) -> usize {
        self.ale.rgb32_size()
    }
    pub fn rgb24_size(&self) -> usize {
        self.ale.rgb24_size()
    }
    pub fn ram_size(&self) -> usize {
        self.ale.ram_size()
    }
    pub fn render_rgb32(&self, buf: &mut [u8]) {
        self.ale.rgb32(buf);
    }
    pub fn render_rgb24(&self, buf: &mut [u8]) {
        self.ale.rgb24(buf);
    }
    pub fn render_ram(&self, buf: &mut [u8]) {
        self.ale.ram(buf);
    }
}

pub struct AtariRamEnv {
    inner: AtariEnv,
}

pub struct AtariRgbEnv {
    inner: AtariEnv,
}

impl DiscreteEnv<u8> for AtariRamEnv {
    fn state(&self, observation: &mut [u8]) { self.inner.state(self, observation); }
    fn step(&mut self, action: ndarray::ArrayD<f32>) -> Result<i32> { 
        let action = action.into_dimensionality::<Ix0>()?.into_scalar() as AtariAction;
        self.inner.step(action)
    }
    fn is_over(&self) -> bool { self.inner.is_game_over(self) }
    fn reset(&mut self) { self.inner.reset(self); }
}

impl DiscreteEnv<u8> for AtariRgbEnv {
    fn state(&self, observation: &mut [u8]) { self.inner.state(self, observation); }
    fn step(&mut self, action: ndarray::ArrayD<f32>) -> Result<i32> { 
        let action = action.into_dimensionality::<Ix0>()?.into_scalar() as AtariAction;
        self.inner.step(action)
    }
    fn is_over(&self) -> bool { self.inner.is_game_over(self) }
    fn reset(&mut self) { self.inner.reset(self); }
}
