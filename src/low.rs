use std::ffi::c_void;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;

use crate::sys::*;
use crate::utils::convert_cstr_to_string;

static SMGN_INITIALIZED: AtomicBool = AtomicBool::new(false);

pub struct Smgn {
    smgn_ptr: *const c_void,
}

impl Smgn {
    pub fn init() -> anyhow::Result<Self> {
        let was_initialized = SMGN_INITIALIZED.swap(true, Ordering::SeqCst);

        if was_initialized {
            anyhow::bail!("Low level smgn is already initialized!");
        }

        let smgn_ptr = unsafe { smgn_init() };

        Ok(Self { smgn_ptr })
    }

    pub fn process_events(&self) {
        unsafe { smgn_events_process(self.smgn_ptr) };
    }

    pub fn should_quit(&self) -> bool {
        unsafe { smgn_get_should_quit(self.smgn_ptr) }
    }

    pub fn create_window(&self) -> anyhow::Result<Window> {
        let sdl_window_ptr = unsafe { smgn_sdl_window_create() };

        if sdl_window_ptr.is_null() {
            anyhow::bail!("Failed to create SDL3 window: {}", get_sdl_error());
        }

        Ok(Window { sdl_window_ptr })
    }
}

impl Drop for Smgn {
    fn drop(&mut self) {
        unsafe { smgn_quit(self.smgn_ptr) };

        SMGN_INITIALIZED.store(false, Ordering::SeqCst);
    }
}

pub struct Window {
    sdl_window_ptr: *const c_void,
}

impl Window {
    pub fn create_renderer(&self) -> anyhow::Result<Renderer> {
        let sdl_renderer_ptr = unsafe { smgn_sdl_renderer_create(self.sdl_window_ptr) };

        if sdl_renderer_ptr.is_null() {
            anyhow::bail!(
                "Failed to crate SDL3 renderer for window {:?}: {}",
                self.sdl_window_ptr,
                get_sdl_error()
            );
        }

        Ok(Renderer { sdl_renderer_ptr })
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { smgn_sdl_window_nuke(self.sdl_window_ptr) };
    }
}

pub struct Renderer {
    sdl_renderer_ptr: *const c_void,
}

impl Renderer {
    pub fn render(&self, r: u8, g: u8, b: u8) {
        unsafe { smgn_sdl_renderer_render(self.sdl_renderer_ptr, r, g, b) };
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe { smgn_sdl_renderer_nuke(self.sdl_renderer_ptr) };
    }
}

pub fn get_sdl_error() -> String {
    let error_string_ptr = unsafe { smgn_get_sdl_error() };

    if error_string_ptr.is_null() {
        panic!("Function smgn_get_sdl_error() returned null pointer!");
    }

    unsafe { convert_cstr_to_string(error_string_ptr) }
}
