use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub rotation: f32,       // radians
    pub texture: Texture2D,
    pub alive: bool,
    shoot_cooldown: f32,
}

impl Player {
    pub async fn new() -> Self {
        let texture = load_texture("assets/toilet.png").await.unwrap();
        texture.set_filter(FilterMode::Linear);
        Self {
            pos: vec2(640.0, 360.0),
            vel: Vec2::ZERO,
            rotation: 0.0,
            texture,
            alive: true,
            shoot_cooldown: 0.0,
        }
    }

    pub fn update(&mut self) -> Option<Vec2> {
        let dt = get_frame_time();

        // Rotation
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.rotation -= 3.0 * dt;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.rotation += 3.0 * dt;
        }

        // Thrust
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            let dir = Vec2::from_angle(self.rotation - std::f32::consts::FRAC_PI_2);
            self.vel += dir * 400.0 * dt;
        }

        // Drag
        self.vel *= 1.0 - 0.02_f32.min(1.0) * (dt * 60.0);

        // Max speed
        if self.vel.length() > 400.0 {
            self.vel = self.vel.normalize() * 400.0;
        }

        self.pos += self.vel * dt;

        // Screen wrap
        let (w, h) = (screen_width(), screen_height());
        if self.pos.x < 0.0 { self.pos.x = w; }
        if self.pos.x > w   { self.pos.x = 0.0; }
        if self.pos.y < 0.0 { self.pos.y = h; }
        if self.pos.y > h   { self.pos.y = 0.0; }

        // Shooting
        self.shoot_cooldown -= dt;
        if (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Z))
            && self.shoot_cooldown <= 0.0
        {
            self.shoot_cooldown = 0.25;
            let dir = Vec2::from_angle(self.rotation - std::f32::consts::FRAC_PI_2);
            return Some(self.pos + dir * 32.0);
        }

        None
    }

    pub fn draw(&self) {
        let size = 64.0;
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

    pub fn radius(&self) -> f32 { 24.0 }
}