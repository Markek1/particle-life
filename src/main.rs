use macroquad::prelude::*;
use std::time;

mod config;
mod game;
mod helpers;
mod particle;
mod quadtree;
mod ui;

use config::*;
use game::*;

fn window_config() -> Conf {
    Conf {
        window_title: "Particle Life".to_owned(),
        window_width: WINDOW_SIZE_PX.x.round() as i32,
        window_height: WINDOW_SIZE_PX.y.round() as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_config)]
async fn main() {
    rand::srand(
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .expect("Time travel")
            .as_secs() as u64,
    );
    let mut game = Game::new();

    loop {
        game.handle_input();

        game.update();

        game.draw();

        println!("{}", get_fps());

        next_frame().await
    }
}
