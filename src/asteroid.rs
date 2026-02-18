use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum AsteroidSize {
    Big,
    Medium,
    Small,
}

impl AsteroidSize {
    pub fn radius(&self) -> f32 {
        match self {
            AsteroidSize::Big    => 56.0,
            AsteroidSize::Medium => 32.0,
            AsteroidSize::Small  => 16.0,
        }
    }

    pub fn draw_size(&self) -> f32 {
        match self {
            AsteroidSize::Big    => 128.0,
            AsteroidSize::Medium => 64.0,
            AsteroidSize::Small  => 32.0,
        }
    }

    pub fn speed(&self) -> f32 {
        match self {
            AsteroidSize::Big    => 60.0,
            AsteroidSize::Medium => 100.0,
            AsteroidSize::Small  => 160.0,
        }
    }

    pub fn score(&self) -> u32 {
        match self {
            AsteroidSize::Big    => 20,
            AsteroidSize::Medium => 50,
            AsteroidSize::Small  => 100,
        }
    }

    /// Returns the two children sizes when split, or None if Small
    pub fn split(&self) -> Option<AsteroidSize> {
        match self {
            AsteroidSize::Big    => Some(AsteroidSize::Medium),
            AsteroidSize::Medium => Some(AsteroidSize::Small),
            AsteroidSize::Small  => None,
        }
    }
}

pub struct Asteroid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub rotation: f32,
    pub rot_speed: f32,
    pub size: AsteroidSize,
    pub texture: Texture2D,
    pub alive: bool,
}

impl Asteroid {
    pub fn new(pos: Vec2, size: AsteroidSize, texture: Texture2D) -> Self {
        let angle = rand::gen_range(0.0_f32, std::f32::consts::TAU);
        let speed = size.speed();
        let rot_speed = rand::gen_range(-2.0_f32, 2.0_f32);

        Self {
            pos,
            vel: Vec2::from_angle(angle) * speed,
            rotation: rand::gen_range(0.0_f32, std::f32::consts::TAU),
            rot_speed,
            size,
            texture,
            alive: true,
        }
    }

    /// Spawn two children after being hit
    pub fn split(&self, tex_medium: &Texture2D, tex_small: &Texture2D) -> Vec<Asteroid> {
        let child_size = match self.size.split() {
            Some(s) => s,
            None => return vec![],
        };
        let tex = match child_size {
            AsteroidSize::Medium => tex_medium.clone(),
            AsteroidSize::Small  => tex_small.clone(),
            _ => unreachable!(),
        };

        (0..2).map(|_| Asteroid::new(self.pos, child_size, tex.clone())).collect()
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.rotation += self.rot_speed * dt;
        self.pos += self.vel * dt;

        let (w, h) = (screen_width(), screen_height());
        if self.pos.x < 0.0 { self.pos.x = w; }
        if self.pos.x > w   { self.pos.x = 0.0; }
        if self.pos.y < 0.0 { self.pos.y = h; }
        if self.pos.y > h   { self.pos.y = 0.0; }
    }

    pub fn draw(&self) {
        let size = self.size.draw_size();
        draw_texture_ex(
            &self.texture,
            self.pos.x - size / 2.0,
            self.pos.y - size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                rotation: self.rotation,
                pivot: Some(self.pos),
                ..Default::default()
            },
        );
    }

    pub fn radius(&self) -> f32 {
        self.size.radius()
    }
}