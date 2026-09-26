use std::ffi::c_char;
use std::ffi::c_void;

unsafe extern "C" {
    pub fn smgn_sdl_window_create() -> *const c_void;
    pub fn smgn_sdl_window_nuke(sdl_window: *const c_void);
    pub fn smgn_sdl_renderer_create(sdl_window: *const c_void) -> *const c_void;
    pub fn smgn_sdl_renderer_nuke(sdl_renderer: *const c_void);
    pub fn smgn_sdl_renderer_render(sdl_renderer: *const c_void, r: u8, g: u8, b: u8);
    pub fn smgn_init() -> *const c_void;
    pub fn smgn_quit(smgn: *const c_void);
    pub fn smgn_get_should_quit(smgn: *const c_void) -> bool;
    pub fn smgn_get_sdl_error() -> *const c_char;
    pub fn smgn_events_process(smgn: *const c_void);
}
