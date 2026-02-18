use macroquad::prelude::*;

// The vertex shader just passes UVs through — all the work is in the fragment shader.
const CRT_VERTEX: &str = r#"
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
"#;

// The fragment shader does all the CRT magic.
const CRT_FRAGMENT: &str = r#"
    #version 100
    precision lowp float;
    varying vec2 uv;
    uniform sampler2D Texture;

    // Bend the UV coordinates to simulate a curved CRT screen.
    // The higher the `bend` value, the more curved it looks.
    vec2 curve(vec2 uv) {
        uv = (uv - 0.5) * 2.0;             // remap to -1..1
        uv *= 1.0 + dot(uv.yx, uv.yx) * 0.04; // apply barrel distortion
        uv = (uv / 2.0) + 0.5;             // remap back to 0..1
        return uv;
    }

    void main() {
        vec2 curved_uv = curve(uv);

        // If the curved UV goes outside 0..1 we're in the black border — draw black.
        if (curved_uv.x < 0.0 || curved_uv.x > 1.0 ||
            curved_uv.y < 0.0 || curved_uv.y > 1.0) {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
            return;
        }

        // Chromatic aberration: sample R, G, B channels at slightly offset UVs.
        // This mimics the color fringing of old CRT tubes.
        float aberration = 0.002;
        float r = texture2D(Texture, curved_uv + vec2( aberration, 0.0)).r;
        float g = texture2D(Texture, curved_uv).g;
        float b = texture2D(Texture, curved_uv - vec2( aberration, 0.0)).b;
        float a = texture2D(Texture, curved_uv).a;
        vec4 color = vec4(r, g, b, a);

        // Scanlines: darken every other horizontal line.
        // mod(gl_FragCoord.y, 2.0) alternates between 0 and 1 each pixel row.
        float scanline = sin(curved_uv.y * 800.0) * 0.04;
        color.rgb -= scanline;

        // Vignette: darken the edges of the screen.
        // smoothstep creates a soft falloff from center to edge.
        float vignette = 16.0 * curved_uv.x * curved_uv.y *
                         (1.0 - curved_uv.x) * (1.0 - curved_uv.y);
        vignette = clamp(pow(vignette, 0.25), 0.0, 1.0);
        color.rgb *= vignette;

        // Slight brightness boost to compensate for the darkening effects
        color.rgb *= 1.1;

        gl_FragColor = color;
    }
"#;

pub struct CrtEffect {
    pub material: Material,
    pub render_target: RenderTarget,
}

impl CrtEffect {
    pub fn new() -> Self {
        // RenderTarget is an off-screen texture we draw the game into first,
        // then apply the shader when drawing it to the real screen.
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

    // Call this BEFORE drawing anything in the frame.
    // Redirects all draw calls to the off-screen render target.
    pub fn begin(&self) {
        set_camera(&Camera2D {
            zoom: vec2(2.0 / screen_width(), 2.0 / screen_height()),
            target: vec2(screen_width() / 2.0, screen_height() / 2.0),
            render_target: Some(self.render_target.clone()),
            ..Default::default()
        });
    }

    // Call this AFTER drawing everything in the frame.
    // Draws the off-screen texture to the real screen with the CRT shader applied.
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
                // Flip Y because render targets are upside down in OpenGL
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}