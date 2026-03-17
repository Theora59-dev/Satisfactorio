mod engine;
mod common;
mod game;

use winit::event_loop::EventLoop;

use crate::engine::core::{application::App, state::State};

fn update(state: &mut State) {
    
}

fn main() {
    println!("Hello, world!");

    env_logger::init();

    let event_loop = EventLoop::with_user_event().build().expect("Failed starting event loop");
    let mut app = App::new(update);
    
    event_loop.run_app(&mut app).expect("Failed starting app");
}

// Tutoriel à voir : https://sotrh.github.io/learn-wgpu/beginner/tutorial7-instancing/
