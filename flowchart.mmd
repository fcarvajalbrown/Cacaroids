flowchart TD
    CT["Cargo.toml\n─────────────\nDefines: project name,\ndependencies (macroquad,\nminiquad), build profile"]

    MN["main.rs\n─────────────\nEntry point.\nSets window config (1280x720).\nCreates Game, runs\nthe main loop:\nupdate() → draw()"]

    GM["game.rs\n─────────────\nOwns everything.\nLoads all textures.\nRuns state machine:\nPlaying / GameOver / Victory.\nHandles all collisions.\nSpawns bullets & asteroids."]

    PL["player.rs\n─────────────\nHandles input (WASD/arrows).\nApplies thrust & drag.\nScreen wrapping.\nReturns bullet spawn pos\nwhen Space is pressed."]

    AS["asteroid.rs\n─────────────\nDefines AsteroidSize:\nBig / Medium / Small.\nEach size has radius,\nspeed, score value.\nSplit logic: Big→2xMedium,\nMedium→2xSmall, Small→dies."]

    BL["bullet.rs\n─────────────\nMoves in a direction\nat fixed speed.\nHas a lifetime timer.\nScreen wrapping.\nMarks itself dead\nwhen lifetime expires."]

    SH["shader.rs\n─────────────\nGLSL CRT shader:\n• Barrel distortion\n• Chromatic aberration\n• Scanlines\n• Vignette\nDraws game to off-screen\ntarget, then applies\nshader to final screen."]

    AS2["assets/\n─────────────\ntoilet.png → player\npoopbig.png → large\npoopmid.png → medium\npoopsmall.png → small\nbullet.png → bullet\nbackground.png → bg"]

    WB["WASM Build\n─────────────\ncargo build\n--target wasm32-unknown-unknown\n--release\n↓\nasteroids_bg.wasm"]

    IT["itch.io ZIP\n─────────────\nindex.html\nmq_js_bundle.js\nasteroids_bg.wasm\nassets/"]

    CT --> MN
    MN --> GM
    GM --> PL
    GM --> AS
    GM --> BL
    GM --> SH
    AS2 --> GM
    GM --> WB
    WB --> IT

    PL -- "returns bullet\nspawn position" --> GM
    AS -- "split() returns\n2 child asteroids" --> GM
    BL -- "alive=false\nwhen expired" --> GM
    SH -- "begin() before draw\nend() after draw" --> GM