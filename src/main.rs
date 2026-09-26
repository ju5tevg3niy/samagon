use std::ffi::c_void;
use std::thread;
use std::time::Duration;

unsafe extern "C" {
    fn smgn_sdl_window_create() -> *const c_void;
    fn smgn_sdl_window_nuke(sdl_window: *const c_void);
    fn smgn_sdl_renderer_create(sdl_window: *const c_void) -> *const c_void;
    fn smgn_sdl_renderer_nuke(sdl_renderer: *const c_void);
    fn smgn_sdl_renderer_render(sdl_renderer: *const c_void, r: u8, g: u8, b: u8);
    fn smgn_init() -> *const c_void;
    fn smgn_quit(smgn: *const c_void);
    fn smgn_get_should_quit(smgn: *const c_void) -> bool;
    fn smgn_events_process(smgn: *const c_void);
}

fn main() {
    let smgn_ptr = unsafe { smgn_init() };
    let window_ptr = unsafe { smgn_sdl_window_create() };
    let renderer_ptr = unsafe { smgn_sdl_renderer_create(window_ptr) };

    let mut r: u8 = 67;
    let mut g: u8 = 128;
    let mut b: u8 = 200;

    loop {
        unsafe { smgn_events_process(smgn_ptr) };

        let should_quit = unsafe { smgn_get_should_quit(smgn_ptr) };

        if should_quit {
            break;
        }

        r = r.wrapping_add(1);
        g = g.wrapping_add(1);
        b = b.wrapping_add(1);

        unsafe { smgn_sdl_renderer_render(renderer_ptr, r, g, b) };

        let sleep_duration = Duration::from_millis(16);
        thread::sleep(sleep_duration);
    }

    unsafe { smgn_sdl_renderer_nuke(renderer_ptr) };
    unsafe { smgn_sdl_window_nuke(window_ptr) };
    unsafe { smgn_quit(smgn_ptr) };
}
