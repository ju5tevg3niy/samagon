use std::thread;
use std::time::Duration;
use std::time::Instant;

use anyhow::Context;
use clap::Parser;

pub mod cli;
pub mod engine;
pub mod project;
pub mod sys;
pub mod utils;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();

    dbg!(&cli);

    use cli::Commands;
    match cli.command {
        Commands::Run => {
            println!("Run command");
        }
        Commands::GameLoopTest => {
            game_loop()?;
        }
        Commands::ReadProject { path } => {
            let project = project::Project::new(&path)
                .with_context(|| format!("Failed to read project from {path:?}"))?;

            println!("The project is: {project:#?}");
        }
        Commands::ReadAsset { path } => {
            let asset = project::assets::Asset::read(&path)
                .with_context(|| format!("Failed to read asset from {path:?}"))?;

            println!("The asset is: {asset:#?}");
        }
    }

    Ok(())
}

fn game_loop() -> anyhow::Result<()> {
    let sdl3 = engine::sdl3::SDL3Wrapper::init().context("Failed to init SDL3")?;

    let window = sdl3.create_window().context("Failed to create window")?;

    println!("The window is: {window:#?}");

    let renderer = window
        .create_renderer()
        .context("Failed to create renderer")?;

    println!("The renderer is: {renderer:#?}");

    let mut engine = engine::core::SmgnEngine::new();

    println!("The engine is: {engine:#?}");

    let mut sdl3_events = sdl3.get_event_provider();

    println!("The event provider is: {sdl3_events:#?}");

    let start = Instant::now();

    println!("Main loop start");

    let test_texture = renderer
        .load_texture("misc/test_data/test_project1/assets/test_guy.png")
        .context("Failed to load test texture")?;

    loop {
        engine.process_events(&mut sdl3_events);

        let elapsed = start.elapsed();
        let t = elapsed.as_secs_f32();

        println!("Time: {t}");

        let r = ((t * 1.0 + 2.0).sin() + 1.0) / 2.0 * 255.0;
        let g = ((t * 3.0 + 123.0).sin() + 1.0) / 2.0 * 255.0;
        let b = ((t * 2.0 + 67.0).sin() + 1.0) / 2.0 * 255.0;

        renderer
            .render_start(r as u8, g as u8, b as u8)
            .context("Failed to start rendering")?;

        renderer
            .render_texture(&test_texture)
            .context("Failed to render test texture")?;

        renderer
            .render_finish()
            .context("Failed to finish rendering")?;

        if engine.should_quit() {
            break;
        }

        let sleep_duration = Duration::from_millis(250);
        thread::sleep(sleep_duration);
    }

    println!("Main loop end");

    Ok(())
}
