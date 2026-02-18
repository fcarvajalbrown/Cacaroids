use macroquad::prelude::*;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::asteroid::{Asteroid, AsteroidSize};

const INITIAL_ASTEROIDS: usize = 5;
const SAFE_RADIUS: f32 = 150.0; // min spawn distance from player

#[derive(PartialEq)]
enum GameState {
    Playing,
    GameOver,
    Victory,
}

pub struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    state: GameState,
    score: u32,

    // textures
    tex_background: Texture2D,
    tex_bullet: Texture2D,
    tex_big: Texture2D,
    tex_medium: Texture2D,
    tex_small: Texture2D,
}

impl Game {
    pub async fn new() -> Self {
        let tex_background = load_texture("assets/background.png").await.unwrap();
        let tex_bullet     = load_texture("assets/bullet.png").await.unwrap();
        let tex_big        = load_texture("assets/poopbig.png").await.unwrap();
        let tex_medium     = load_texture("assets/poopmid.png").await.unwrap();
        let tex_small      = load_texture("assets/poopsmall.png").await.unwrap();
        let score_text = format!("SCORE: {}", self.score);
        let text_size = measure_text(&score_text, None, 32, 1.0);
        let pad_x = 12.0;
        let pad_y = 8.0;
        let rx = 15.0;
        let ry = 15.0;
        let bx = rx - pad_x;
        let by = ry - pad_y;
        let bw = text_size.width + pad_x * 2.0;
        let bh = text_size.height + pad_y * 2.0;

        for tex in [&tex_background, &tex_bullet, &tex_big, &tex_medium, &tex_small] {
            tex.set_filter(FilterMode::Linear);
        }

        let player = Player::new().await;
        let asteroids = Self::spawn_asteroids(INITIAL_ASTEROIDS, player.pos, &tex_big);

        Self {
            player,
            bullets: vec![],
            asteroids,
            state: GameState::Playing,
            score: 0,
            tex_background,
            tex_bullet,
            tex_big,
            tex_medium,
            tex_small,
        }
    }

    fn spawn_asteroids(count: usize, avoid: Vec2, tex: &Texture2D) -> Vec<Asteroid> {
        (0..count).map(|_| {
            loop {
                let pos = vec2(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                );
                if pos.distance(avoid) > SAFE_RADIUS {
                    return Asteroid::new(pos, AsteroidSize::Big, tex.clone());
                }
            }
        }).collect()
    }

    pub fn update(&mut self) {
        if self.state != GameState::Playing {
            if is_key_pressed(KeyCode::R) {
                self.restart();
            }
            return;
        }

        // Player update — may spawn bullet
        if let Some(bullet_pos) = self.player.update() {
            let dir = Vec2::from_angle(self.player.rotation - std::f32::consts::FRAC_PI_2);
            self.bullets.push(Bullet::new(bullet_pos, dir, self.tex_bullet.clone()));
        }

        // Bullets update
        for b in self.bullets.iter_mut() {
            b.update();
        }

        // Asteroids update
        for a in self.asteroids.iter_mut() {
            a.update();
        }

        // Bullet <-> Asteroid collisions
        let mut new_asteroids: Vec<Asteroid> = vec![];
        for b in self.bullets.iter_mut() {
            if !b.alive { continue; }
            for a in self.asteroids.iter_mut() {
                if !a.alive { continue; }
                if b.pos.distance(a.pos) < b.radius() + a.radius() {
                    b.alive = false;
                    a.alive = false;
                    self.score += a.size.score();
                    let children = a.split(&self.tex_medium, &self.tex_small);
                    new_asteroids.extend(children);
                }
            }
        }
        self.asteroids.extend(new_asteroids);

        // Player <-> Asteroid collisions
        if self.player.alive {
            for a in self.asteroids.iter() {
                if !a.alive { continue; }
                if self.player.pos.distance(a.pos) < self.player.radius() + a.radius() {
                    self.player.alive = false;
                    self.state = GameState::GameOver;
                    return;
                }
            }
        }

        // Cleanup dead entities
        self.bullets.retain(|b| b.alive);
        self.asteroids.retain(|a| a.alive);

        // Victory condition
        if self.asteroids.is_empty() {
            self.state = GameState::Victory;
        }
    }

    pub fn draw(&self) {
        // Background
        draw_texture_ex(
            &self.tex_background,
            0.0, 0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Game objects
        for a in self.asteroids.iter().filter(|a| a.alive) {
            a.draw();
        }
        for b in self.bullets.iter().filter(|b| b.alive) {
            b.draw();
        }
        if self.player.alive {
            self.player.draw();
        }

        // HUD
        draw_rectangle(bx, by, bw, bh, Color::new(0.0, 0.0, 0.0, 0.6));
        draw_text(&format!("SCORE: {}", self.score), 20.0, 40.0, 32.0, WHITE);

        // Overlays
        match self.state {
            GameState::GameOver => self.draw_overlay("GAME OVER", "Press R to restart"),
            GameState::Victory  => self.draw_overlay("YOU WIN!", "Press R to play again"),
            _ => {}
        }
    }

    fn draw_overlay(&self, title: &str, subtitle: &str) {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));
        let tw = measure_text(title, None, 64, 1.0).width;
        draw_text(title, cx - tw / 2.0, cy - 20.0, 64.0, WHITE);
        let sw = measure_text(subtitle, None, 32, 1.0).width;
        draw_text(subtitle, cx - sw / 2.0, cy + 30.0, 32.0, LIGHTGRAY);
    }

    fn restart(&mut self) {
        self.bullets.clear();
        self.score = 0;
        self.asteroids = Self::spawn_asteroids(INITIAL_ASTEROIDS, vec2(640.0, 360.0), &self.tex_big);
        // re-init player in place without reloading texture
        self.player.pos = vec2(640.0, 360.0);
        self.player.vel = Vec2::ZERO;
        self.player.rotation = 0.0;
        self.player.alive = true;
        self.state = GameState::Playing;
    }
}