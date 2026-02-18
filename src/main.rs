use macroquad::prelude::*;

mod game;
mod player;
mod asteroid;
mod bullet;

use game::Game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Poop Asteroids".to_string(),
        window_width: 1280,
        window_height: 720,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    loop {
        game.update();
        game.draw();
        next_frame().await;
    }
}