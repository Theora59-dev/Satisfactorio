mod engine;
mod world;
mod player;

use engine::core::application::run;

fn main() {
    println!("Hello, world!");
    let _ = run();
}

// Tutoriel à voir : https://sotrh.github.io/learn-wgpu/beginner/tutorial7-instancing/
