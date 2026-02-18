use macroquad::prelude::*;
use crate::player::Player;
use crate::bullet::Bullet;
use crate::asteroid::{Asteroid, AsteroidSize};

// How many big asteroids spawn at the start of each game
const INITIAL_ASTEROIDS: usize = 5;

// Minimum distance from the player where asteroids can spawn.
// Prevents instant death at game start.
const SAFE_RADIUS: f32 = 150.0;

// The game can be in one of these three states.
// This drives what gets updated and what gets drawn.
#[derive(PartialEq)]
enum GameState {
    Playing,
    GameOver,
    Victory,
}

// The central struct that owns everything in the game.
// All textures, all entities, all state lives here.
pub struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    asteroids: Vec<Asteroid>,
    state: GameState,
    score: u32,

    // Textures are stored here and cloned (cheap, ref-counted) into entities.
    // This means we only upload each image to the GPU once.
    tex_background: Texture2D,
    tex_bullet: Texture2D,
    tex_big: Texture2D,
    tex_medium: Texture2D,
    tex_small: Texture2D,
}

impl Game {
    // Async because macroquad's texture loading is async (works on both native and WASM).
    pub async fn new() -> Self {
        // Load all textures from the assets/ folder next to the executable.
        let tex_background = load_texture("assets/background.png").await.unwrap();
        let tex_bullet     = load_texture("assets/bullet.png").await.unwrap();
        let tex_big        = load_texture("assets/poopbig.png").await.unwrap();
        let tex_medium     = load_texture("assets/poopmid.png").await.unwrap();
        let tex_small      = load_texture("assets/poopsmall.png").await.unwrap();

        // Linear filtering = smooth scaling. Use Nearest if you want pixel-art crispness.
        for tex in [&tex_background, &tex_bullet, &tex_big, &tex_medium, &tex_small] {
            tex.set_filter(FilterMode::Linear);
        }

        let player = Player::new().await;

        // Spawn initial asteroids avoiding the player's starting position
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

    // Spawns `count` big asteroids at random positions,
    // retrying each one until it's far enough from `avoid`.
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
                // If too close, loop again and try a new random position
            }
        }).collect()
    }

    // Called every frame. Handles input, physics, and collision detection.
    pub fn update(&mut self) {
        // If not playing, only listen for restart input
        if self.state != GameState::Playing {
            if is_key_pressed(KeyCode::R) {
                self.restart();
            }
            return;
        }

        // --- PLAYER UPDATE ---
        // player.update() returns Some(pos) if the player fired a bullet this frame
        if let Some(bullet_pos) = self.player.update() {
            // Compute the forward direction from the player's current rotation
            let dir = Vec2::from_angle(self.player.rotation - std::f32::consts::FRAC_PI_2);
            self.bullets.push(Bullet::new(bullet_pos, dir, self.tex_bullet.clone()));
        }

        // --- BULLET UPDATE ---
        for b in self.bullets.iter_mut() {
            b.update();
        }

        // --- ASTEROID UPDATE ---
        for a in self.asteroids.iter_mut() {
            a.update();
        }

        // --- BULLET <-> ASTEROID COLLISIONS ---
        // We collect new child asteroids separately to avoid mutating
        // the vec while iterating over it (Rust won't allow that).
        let mut new_asteroids: Vec<Asteroid> = vec![];

        for b in self.bullets.iter_mut() {
            if !b.alive { continue; } // skip already-dead bullets

            for a in self.asteroids.iter_mut() {
                if !a.alive { continue; } // skip already-dead asteroids

                // Simple circle-circle collision check
                if b.pos.distance(a.pos) < b.radius() + a.radius() {
                    b.alive = false; // bullet is consumed
                    a.alive = false; // asteroid is destroyed
                    self.score += a.size.score();

                    // Split into 2 smaller asteroids (or nothing if already Small)
                    let children = a.split(&self.tex_medium, &self.tex_small);
                    new_asteroids.extend(children);
                }
            }
        }

        // Now it's safe to add the new asteroids
        self.asteroids.extend(new_asteroids);

        // --- PLAYER <-> ASTEROID COLLISIONS ---
        if self.player.alive {
            for a in self.asteroids.iter() {
                if !a.alive { continue; }
                if self.player.pos.distance(a.pos) < self.player.radius() + a.radius() {
                    self.player.alive = false;
                    self.state = GameState::GameOver;
                    return; // no need to check further
                }
            }
        }

        // --- CLEANUP ---
        // Remove entities that were marked dead this frame.
        // retain() keeps only elements where the closure returns true.
        self.bullets.retain(|b| b.alive);
        self.asteroids.retain(|a| a.alive);

        // --- VICTORY CHECK ---
        // Player cleared all asteroids including all split children
        if self.asteroids.is_empty() {
            self.state = GameState::Victory;
        }
    }

    // Called every frame after update(). Pure rendering, no logic here.
    pub fn draw(&self) {
        // --- BACKGROUND ---
        // Stretch the background texture to fill the entire window
        draw_texture_ex(
            &self.tex_background,
            0.0, 0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // --- ASTEROIDS ---
        for a in self.asteroids.iter().filter(|a| a.alive) {
            a.draw();
        }

        // --- BULLETS ---
        for b in self.bullets.iter().filter(|b| b.alive) {
            b.draw();
        }

        // --- PLAYER ---
        if self.player.alive {
            self.player.draw();
        }

        // --- HUD: SCORE ---
        // Draw a semi-transparent dark background behind the score text
        // so it's readable over any background color.
        let score_text = format!("SCORE: {}", self.score);
        let text_size = measure_text(&score_text, None, 32, 1.0);
        let pad_x = 12.0;
        let pad_y = 8.0;
        let rx = 15.0; // text x position
        let ry = 15.0; // text y position (top of box)
        let bx = rx - pad_x;
        let by = ry - pad_y;
        let bw = text_size.width + pad_x * 2.0;
        let bh = text_size.height + pad_y * 2.0;

        // Background panel first, then text on top
        draw_rectangle(bx, by, bw, bh, Color::new(0.0, 0.0, 0.0, 0.6));
        draw_text(&score_text, rx, ry + text_size.height, 32.0, WHITE);

        // --- OVERLAYS (Game Over / Victory) ---
        match self.state {
            GameState::GameOver => self.draw_overlay("GAME OVER", "Press R to restart"),
            GameState::Victory  => self.draw_overlay("YOU WIN!", "Press R to play again"),
            GameState::Playing  => {}
        }
    }

    // Draws a centered fullscreen dim overlay with a title and subtitle.
    // Used for Game Over and Victory screens.
    fn draw_overlay(&self, title: &str, subtitle: &str) {
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;

        // Semi-transparent black overlay over the whole screen
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.6));

        // Center the title text horizontally
        let tw = measure_text(title, None, 64, 1.0).width;
        draw_text(title, cx - tw / 2.0, cy - 20.0, 64.0, WHITE);

        // Center the subtitle text horizontally
        let sw = measure_text(subtitle, None, 32, 1.0).width;
        draw_text(subtitle, cx - sw / 2.0, cy + 30.0, 32.0, LIGHTGRAY);
    }

    // Resets all game state back to initial conditions without reloading textures.
    // Textures are just cloned (ref-counted pointer copy) so this is fast.
    fn restart(&mut self) {
        self.bullets.clear();
        self.score = 0;
        self.asteroids = Self::spawn_asteroids(INITIAL_ASTEROIDS, vec2(640.0, 360.0), &self.tex_big);

        // Reset player state manually — avoids reloading the texture from disk
        self.player.pos = vec2(640.0, 360.0);
        self.player.vel = Vec2::ZERO;
        self.player.rotation = 0.0;
        self.player.alive = true;

        self.state = GameState::Playing;
    }
}