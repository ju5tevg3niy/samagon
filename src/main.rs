unsafe extern "C" {
    fn smgn_sdl_init();
    fn smgn_sdl_quit();
}

fn main() {
    unsafe { smgn_sdl_init() };
    unsafe { smgn_sdl_quit() };
}
