mod engine;
mod common;
mod game;

use winit::event_loop::EventLoop;

use crate::engine::core::application::App;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();

    event_loop.run_app(&mut app)?;

    Ok(())
}

// Tutoriel à voir : https://sotrh.github.io/learn-wgpu/beginner/tutorial7-instancing/
