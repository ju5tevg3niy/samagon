use std::thread;
use std::time::Duration;

use anyhow::Context;

mod low;
mod sys;
mod utils;

fn main() -> anyhow::Result<()> {
    let smgn = low::Smgn::init().context("Failed to init Smgn")?;
    let window = smgn.create_window().context("Failed to create window")?;
    let renderer = window
        .create_renderer()
        .context("Failed to create renderer")?;

    let mut t: f32 = 0.0;

    loop {
        smgn.process_events();

        let should_quit = smgn.should_quit();

        if should_quit {
            break;
        }

        let r = ((t * 1.0 + 2.0).sin() + 1.0) / 2.0 * 255.0;
        let g = ((t * 3.0 + 123.0).sin() + 1.0) / 2.0 * 255.0;
        let b = ((t * 2.0 + 67.0).sin() + 1.0) / 2.0 * 255.0;

        renderer.render(r as u8, g as u8, b as u8);

        let sleep_duration = Duration::from_millis(16);
        thread::sleep(sleep_duration);
        t += 0.016;
    }

    std::mem::drop(smgn);

    let _smgn2 = low::Smgn::init().context("Failed to init Smgn")?;

    Ok(())
}
