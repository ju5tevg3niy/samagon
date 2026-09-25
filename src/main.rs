use std::ffi::c_void;
use std::thread;
use std::time::Duration;

unsafe extern "C" {
    fn smgn_sdl_init();
    fn smgn_sdl_quit();
    fn smgn_sdl_get_window() -> *const c_void;
    fn smgn_sdl_get_renderer(sdl_window: *const c_void) -> *const c_void;
    fn smgn_sdl_nuke_window(sdl_window: *const c_void);
    fn smgn_sdl_nuke_renderer(sdl_renderer: *const c_void);
    fn smgn_sdl_render(sdl_renderer: *const c_void, r: u8, g: u8, b: u8);
    fn smgn_init() -> *const c_void;
    fn smgn_quit(smgn: *const c_void);
    fn smgn_process_events(smgn: *const c_void);
    fn smgn_get_should_quit(smgn: *const c_void) -> bool;
}

fn main() {
    unsafe { smgn_sdl_init() };

    let window_ptr = unsafe { smgn_sdl_get_window() };
    let renderer_ptr = unsafe { smgn_sdl_get_renderer(window_ptr) };
    let smgn_ptr = unsafe { smgn_init() };

    let mut r: u8 = 67;
    let mut g: u8 = 128;
    let mut b: u8 = 200;

    loop {
        unsafe { smgn_process_events(smgn_ptr) };

        let should_quit = unsafe { smgn_get_should_quit(smgn_ptr) };

        if should_quit {
            break;
        }

        r = r.wrapping_add(1);
        g = g.wrapping_add(1);
        b = b.wrapping_add(1);

        unsafe { smgn_sdl_render(renderer_ptr, r, g, b) };

        let sleep_duration = Duration::from_millis(16);
        thread::sleep(sleep_duration);
    }

    unsafe { smgn_sdl_nuke_renderer(renderer_ptr) };
    unsafe { smgn_sdl_nuke_window(window_ptr) };
    unsafe { smgn_quit(smgn_ptr) };
    unsafe { smgn_sdl_quit() };
}
