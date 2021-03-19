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

    pub fn step(&self, action: i32) {
        let ret = self.ale.step();
        println!("step return: {}", ret);
    }

    pub fn render(&self) {
        let w: u32 = self.ale.width();
        let h: u32 = self.ale.width();
        let mut buf = vec![0; (w * h * 3) as usize];

        unsafe {
            atari_env_sys::getScreenRGB2(self.ale, buf.as_mut_ptr());
        }

        let img = image::DynamicImage::ImageRgb8(image::RgbImage::from_raw(w, h, buf).unwrap());

        viuer::print(
            &img,
            &viuer::Config {
                x: 0,
                y: 0,
                use_kitty: true,
                width: Some(w),
                height: Some(h),
                ..Default::default()
            },
        );
    }
}
