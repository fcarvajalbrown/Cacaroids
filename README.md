# 💩 Poop Asteroids

A browser-playable Asteroids clone built with **Rust + WebAssembly**, featuring hand-drawn sprites and a CRT post-processing shader.

Built as a learning project to explore Rust game development with [macroquad](https://macroquad.rs/).

---

## 🎮 How to Play

| Key | Action |
|---|---|
| `W` / `↑` | Thrust |
| `A` / `←` | Rotate left |
| `D` / `→` | Rotate right |
| `Space` / `Z` | Shoot |
| `R` | Restart |

Destroy all the poops. Don't get hit. That's it.

---

## 🦀 Tech Stack

- **Language:** Rust
- **Framework:** [macroquad 0.4](https://macroquad.rs/) — a simple Rust game library that compiles to both native and WASM with no extra configuration
- **Rendering:** OpenGL via miniquad (macroquad's backend)
- **Shader:** Custom GLSL CRT post-processing effect (barrel distortion, scanlines, chromatic aberration, vignette)
- **Target:** WebAssembly (`wasm32-unknown-unknown`) for browser play

---

## 🏗️ Project Structure

```
asteroids/
├── src/
│   ├── main.rs        # Entry point, window config
│   ├── game.rs        # Game loop, state machine, collision detection
│   ├── player.rs      # Player movement, shooting, screen wrap
│   ├── asteroid.rs    # Asteroid sizes, splitting logic
│   ├── bullet.rs      # Bullet movement, lifetime
│   └── shader.rs      # CRT effect via render target + GLSL
├── assets/            # PNG sprites (swap these freely)
├── index.html         # WASM loader for itch.io
├── .gitignore
└── Cargo.toml
```

---

## 🔧 Build & Run

**Prerequisites:** [Rust](https://rustup.rs/) installed.

**Run natively:**
```bash
cargo run
```

**Build for WebAssembly:**
```bash
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Then copy `target/wasm32-unknown-unknown/release/asteroids.wasm` to the project root as `asteroids_bg.wasm`, zip it with `index.html` and `assets/`, and upload to itch.io as an HTML game.

---

## 🎨 Swapping Sprites

All sprites live in `assets/`. Replace any PNG and the game picks it up automatically on next build. Expected files:

| File | Used for |
|---|---|
| `toilet.png` | Player ship |
| `poopbig.png` | Large asteroid |
| `poopmid.png` | Medium asteroid |
| `poopsmall.png` | Small asteroid |
| `bullet.png` | Bullet |
| `background.png` | Background (1280×720) |

---

## 📚 Interesting bits for Rust learners

- **No ECS, no engine** — entities are plain structs with `update()` and `draw()` methods, kept in `Vec<T>` in the game state. Simple and readable.
- **Texture sharing** — `Texture2D` in macroquad is ref-counted, so cloning it is cheap. One GPU upload, many references.
- **WASM with zero JS** — macroquad handles the entire WASM/JS bridge. You write pure Rust and it just works in the browser.
- **GLSL shader** — the CRT effect uses a render target: the whole game draws off-screen first, then that texture gets drawn to the real screen with the shader applied.
- **Collision detection** — simple circle vs circle: `distance(a, b) < radius_a + radius_b`. No physics library needed for a game like this.