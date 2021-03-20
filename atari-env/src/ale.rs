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
    pub fn new(rom_path: &Path) -> Self {
        use std::os::unix::ffi::OsStrExt;
        let rom_path = CString::new(rom_path.as_os_str().as_bytes()).unwrap();
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

    pub fn take_action(&self, action: i32) -> i32 {
        let ret: ::std::os::raw::c_int = unsafe { atari_env_sys::act(self.inner, action) };
        ret.into()
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

    pub fn rgb24_size(&self) -> usize {
        return (self.width() as usize) * (self.height() as usize) * 3;
    }
    /// bgr on little-endian, rgb on big-endian
    pub fn rgb24_native_endian(&self, buf: &mut [u8]) {
        unsafe {
            atari_env_sys::getScreenRGB(self.inner, buf.as_mut_ptr());
        }
    }
    /// always rgb in regardless of endianness
    pub fn rgb24(&self, buf: &mut [u8]) {
        unsafe {
            atari_env_sys::getScreenRGB2(self.inner, buf.as_mut_ptr());
        }
    }

    /// always rgb in regardless of endianness
    pub fn rgb32(&self, buf: &mut [u8]) {
        let n = buf.len() / 4;
        self.rgb24(&mut buf[n..]);
        for i in 0..n {
            buf[i * 4 + 0] = buf[n + (i * 3) + 0];
            buf[i * 4 + 1] = buf[n + (i * 3) + 1];
            buf[i * 4 + 2] = buf[n + (i * 3) + 2];
            buf[i * 4 + 3] = 0;
        }
    }

    pub fn save_png<P: AsRef<Path>>(&self, path: P) {
        use std::os::unix::ffi::OsStrExt;
        let path = path.as_ref();
        unsafe {
            atari_env_sys::saveScreenPNG(
                self.inner,
                CString::new(path.as_os_str().as_bytes()).unwrap().as_ptr(),
            );
        }
    }
}
