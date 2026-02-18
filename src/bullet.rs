use macroquad::prelude::*;

pub struct Bullet {
    pub pos: Vec2,
    pub vel: Vec2,
    pub texture: Texture2D,
    pub alive: bool,
    lifetime: f32,
}

impl Bullet {
    pub fn new(pos: Vec2, direction: Vec2, texture: Texture2D) -> Self {
        Self {
            pos,
            vel: direction * 600.0,
            texture,
            alive: true,
            lifetime: 1.5,
        }
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.lifetime -= dt;
        if self.lifetime <= 0.0 {
            self.alive = false;
            return;
        }

        self.pos += self.vel * dt;

        // Screen wrap
        let (w, h) = (screen_width(), screen_height());
        if self.pos.x < 0.0 { self.pos.x = w; }
        if self.pos.x > w   { self.pos.x = 0.0; }
        if self.pos.y < 0.0 { self.pos.y = h; }
        if self.pos.y > h   { self.pos.y = 0.0; }
    }

    pub fn draw(&self) {
        let size = 8.0;
        draw_texture_ex(
            &self.texture,
            self.pos.x - size / 2.0,
            self.pos.y - size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }

    pub fn radius(&self) -> f32 { 4.0 }
}