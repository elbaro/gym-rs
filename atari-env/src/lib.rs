pub mod ale;
use std::path::Path;

pub use ale::Ale;
pub use ale::AleAction as AtariAction;
pub use ale::AleConfig as EmulatorConfig;

use gym_core::{GymEnv, DiscreteEnv};
use ndarray::{Array1, Array3, ArrayView, ArrayD, Ix0, IxDyn};

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
    pub fn into_ram_env(self) -> AtariRamEnv {
        AtariRamEnv::new(self)
    }
    pub fn into_rgb_env(self) -> AtariRgbEnv {
        AtariRgbEnv::new(self)
    }
}

pub struct AtariRamEnv {
    buf1: Array1<u8>,
    buf2: Array1<f32>,
    inner: AtariEnv,
}

pub struct AtariRgbEnv {
    buf1: Array1<u8>,
    buf2: Array3<f32>,
    inner: AtariEnv,
}

impl DiscreteEnv for AtariRamEnv {
    fn new(env: AtariEnv) -> Self {
        Self {
            buf1: Array1::zeros((env.ram_size(),),),
            buf2: Array1::zeros((env.ram_size(),),),
            inner: env,
        }
    }
    fn state(&self) -> ArrayView<_, f32, IxDyn> {
        self.buf2.view();
    }
    fn step(&mut self, action: ArrayD<f32>) -> Result<i32> { 
        let action = action.into_dimensionality::<Ix0>()?.into_scalar() as AtariAction;
        let reward = self.inner.step(action);
        self.inner.state(self.buf1.as_slice_mut());
        ndarray::parallel::par_azip!((a in &mut self.buf2, &b in &self.buf1) {*a = b as f32 / 255.0;});
        Ok(reward)
    }
    fn is_over(&self) -> bool { self.inner.is_game_over(self) }
    fn reset(&mut self) { self.inner.reset(self); }
}

impl DiscreteEnv for AtariRgbEnv {
    fn new(env: AtariEnv) -> Self {
        Self {
            buf1: Array1::zeros(env.rgb24_size()),
            buf2: Array1::zeros((env.height(), env.width(), 3)),
            inner: env,
        }
    }
    fn state(&self) -> ArrayView<_, f32, IxDyn>{
        self.buf2.view()
    }
    fn step(&mut self, action: ArrayD<f32>) -> Result<i32> { 
        let action = action.into_dimensionality::<Ix0>()?.into_scalar() as AtariAction;
        let reward = self.inner.step(action);
        self.inner.state(self.buf.as_slice_mut());
        ndarray::parallel::par_azip!((a in &mut self.buf2, &b in &self.buf1) {*a = b as f32 / 255.0;});
        Ok(reward)
    }
    fn is_over(&self) -> bool { self.inner.is_game_over(self) }
    fn reset(&mut self) { self.inner.reset(self); }
}
