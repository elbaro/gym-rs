mod ale;
mod game;
use ale::Ale;
use game::Game;

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
    pub fn new(
        game: String,
        render_mode: RenderMode,
        // difficulty: i32,
        observation_type: ObservationType,
        frameskip_range: (u8, u8),
    ) -> Self {
        Self {
            _game: game,
            _render_mode: render_mode,
            _observation_type: observation_type,
            _frameskip_range: frameskip_range,
            ale: Ale::new(Game::Tetris),
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
