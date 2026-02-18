use macroquad::prelude::*;

// The vertex shader just passes UV coordinates through to the fragment shader.
// No transformation magic here — all the visual work happens in the fragment shader.
const CRT_VERTEX: &str = "
#version 100
attribute vec3 position;
attribute vec2 texcoord;
varying lowp vec2 uv;
uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1.0);
    uv = texcoord;
}
";

// The fragment shader applies all CRT effects:
// 1. Barrel distortion (curved screen)
// 2. Chromatic aberration (color fringing)
// 3. Scanlines
// 4. Vignette (dark edges)
const CRT_FRAGMENT: &str = "
#version 100
precision lowp float;
varying vec2 uv;
uniform sampler2D Texture;

// Bends UV coords to simulate a curved CRT tube.
// Increase the 0.15 multiplier for more extreme curvature.
vec2 curve(vec2 uv) {
    uv = (uv - 0.5) * 2.0;
    uv *= 1.0 + dot(uv.yx, uv.yx) * 0.04;
    uv = (uv / 2.0) + 0.5;
    return uv;
}

void main() {
    vec2 curved_uv = curve(uv);

    // Anything outside 0..1 after curving is the black border around the screen
    if (curved_uv.x < 0.0 || curved_uv.x > 1.0 ||
        curved_uv.y < 0.0 || curved_uv.y > 1.0) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        return;
    }

    // Chromatic aberration: sample R/G/B at slightly different UV offsets.
    // Mimics the color misalignment of old CRT phosphor guns.
    float aberration = 0.0015;
    float r = texture2D(Texture, curved_uv + vec2( aberration, 0.0)).r;
    float g = texture2D(Texture, curved_uv).g;
    float b = texture2D(Texture, curved_uv - vec2( aberration, 0.0)).b;
    float a = texture2D(Texture, curved_uv).a;
    vec4 color = vec4(r, g, b, a);

    // Scanlines: creates horizontal dark bands like a real CRT.
    // 800.0 controls line frequency — raise for denser lines.
    // 0.15 controls darkness — raise for more contrast.
    float scanline = sin(curved_uv.y * 800.0) * 0.15;
    color.rgb -= scanline;

    // Vignette: darkens screen edges, brighter in the center.
    // Lower the pow() exponent for a stronger, wider dark border.
    float vignette = 16.0 * curved_uv.x * curved_uv.y *
                     (1.0 - curved_uv.x) * (1.0 - curved_uv.y);
    vignette = clamp(pow(vignette, 0.15), 0.0, 1.0);
    color.rgb *= vignette;

    // Slight brightness boost to compensate for the overall darkening
    color.rgb *= 1.2;

    gl_FragColor = color;
}
";

pub struct CrtEffect {
    pub material: Material,
    pub render_target: RenderTarget,
}

impl CrtEffect {
    pub fn new() -> Self {
        // RenderTarget is an off-screen texture we draw the whole game into first.
        // Then we draw THAT texture to the real screen with the CRT shader applied.
        let render_target = render_target(1280, 720);
        render_target.texture.set_filter(FilterMode::Linear);

        let material = load_material(
            ShaderSource::Glsl {
                vertex: CRT_VERTEX,
                fragment: CRT_FRAGMENT,
            },
            MaterialParams::default(),
        ).unwrap();

        Self { material, render_target }
    }

    // Call BEFORE drawing anything in the frame.
    // Redirects all draw calls to the off-screen render target.
    pub fn begin(&self) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            target: vec2(screen_width() / 2.0, screen_height() / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });
    }

    // Call AFTER drawing everything in the frame.
    // Applies the CRT shader and draws the result to the real screen.
    pub fn end(&self) {
        // Switch back to the real screen
        set_default_camera();

        gl_use_material(&self.material);
        draw_texture_ex(
            &self.render_target.texture,
            0.0, 0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                // Render targets are flipped vertically in OpenGL — flip_y fixes that
                flip_y: false,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}