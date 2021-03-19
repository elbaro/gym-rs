use crate::game::Game;
use std::ffi::CString;
use std::path::Path;

pub struct Ale {
    inner: *mut atari_env_sys::ALEInterface,
}

impl Drop for Ale {
    fn drop(&mut self) {
        unsafe {
            atari_env_sys::ALE_del(self.inner);
        }
    }
}

impl Ale {
    pub fn new(game: Game) -> Self {
        let rom_path = CString::new("tetris.bin").unwrap();
        let ale = unsafe { atari_env_sys::ALE_new() };
        unsafe {
            atari_env_sys::loadROM(ale, rom_path.as_ptr());
        }

        Self { inner: ale }
    }
    pub fn available_actions(&self) -> Vec<i32> {
        let n = unsafe { atari_env_sys::getLegalActionSize(self.inner) } as usize;
        let mut buf = vec![0; n];
        unsafe {
            atari_env_sys::getLegalActionSet(self.inner, buf.as_mut_ptr());
        }
        buf
    }

    pub fn lives(&self) -> u32 {
        unsafe { atari_env_sys::lives(self.inner) as u32 }
    }

    pub fn width(&self) -> u32 {
        unsafe { atari_env_sys::getScreenWidth(self.inner) as u32 }
    }
    pub fn height(&self) -> u32 {
        unsafe { atari_env_sys::getScreenHeight(self.inner) as u32 }
    }

    pub fn rgb32_size(&self) -> usize {
        return (self.width() as usize) * (self.height() as usize) * 4;
    }
    pub fn rgb32(&self, buf: &mut [u8]) {
        unsafe {
            atari_env_sys::getScreenRGB(self.inner, buf.as_mut_ptr());
        }
    }

    pub fn rgb24_size(&self) -> usize {
        return (self.width() as usize) * (self.height() as usize) * 3;
    }
    pub fn rgb24(&self, buf: &mut [u8]) {
        unsafe {
            atari_env_sys::getScreenRGB2(self.inner, buf.as_mut_ptr());
        }
    }

    pub fn save_png<P: AsRef<Path>>(&self, path: P) {
        let path = path.as_ref();
        unsafe {
            atari_env_sys::saveScreenPNG(self.inner, path.as_ptr());
        }
    }
}
