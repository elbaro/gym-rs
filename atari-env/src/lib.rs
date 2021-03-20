mod ale;
mod game;
use ale::Ale;

pub struct AtariEnv {
    _game: String,
    _render_mode: RenderMode,
    _observation_type: ObservationType,
    _frameskip_range: (u8, u8),
    ale: Ale,
}

pub enum RenderMode {
    Human,
    RgbArray,
}

pub enum ObservationType {
    Ram,
    Image,
}

impl AtariEnv {
    /// about frame-skipping and action-repeat,
    /// see https://danieltakeshi.github.io/2016/11/25/frame-skipping-and-preprocessing-for-deep-q-networks-on-atari-2600-games/
    pub fn new<P: AsRef<Path>>(
        rom_path: P,
        render_mode: RenderMode,
        // difficulty: i32,
        observation_type: ObservationType,
        frameskip_range: (u8, u8),
    ) -> Self {
        Self {
            _game: game,
            _frameskip_range: frameskip_range,
            ale: Ale::new(rom_path.as_ref()),
        }
    }

    pub fn width(&self) -> u32 {
        self.ale.width()
    }
    pub fn height(&self) -> u32 {
        self.ale.height()
    }

    pub fn step(&self, action: i32) -> i32 {
        let ret = self.ale.take_action(action);
        ret
    }

    pub fn render_rgb32(&self, buf: &mut [u8]) {
        self.ale.rgb32(buf);
    }
    pub fn render_rgb24(&self, buf: &mut [u8]) {
        self.ale.rgb24(buf);
    }
}
