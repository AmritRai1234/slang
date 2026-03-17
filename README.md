# ⚡ Slang — Simple Language for Animated Videos

> Write animations in plain English. Render to video.

Slang is an ultra-simple, English-like programming language for creating animated educational videos — like Manim, but you just write sentences.

```
scene "Hello World"
set background to dark blue

draw a circle at center with radius 100 color cyan
fade in the circle over 1 second

write "Hello, Slang!" at top color white size 48 over 1 second

move the circle to right over 2 seconds easing bounce

fade out everything over 1 second
```

---

## 🚀 Quick Start

```bash
# Build
cargo build --release

# Launch the IDE (split-screen editor + live preview)
cargo run -- ide

# Render a .sl file to PNG frames
cargo run -- render examples/demo.sl --output frames --frames

# Render to MP4 (requires FFmpeg)
cargo run -- render examples/demo.sl --output video.mp4

# Create a new project
cargo run -- new myproject
```

---

## 📖 Language Reference

### Scenes

```
scene "Title"           # start a named scene
next scene              # transition to next scene
```

### Settings

```
set background to dark blue
set resolution to 1920x1080
set fps to 30
set background to gradient blue purple
set background to noise dark blue
set background to radial dark blue black
```

### Drawing Shapes

```
draw a circle at center with radius 100 color cyan
draw a square at left with size 80 color yellow
draw a rectangle at right with size 120 color green
draw a triangle at top with size 100 color pink
draw a line from (100, 200) to (500, 200) color white thickness 3
draw an arrow from (100, 300) to (600, 300) color red thickness 2
```

### Text

```
draw text "Hello" at center color white size 48
write "Animated text" at top color white size 36 over 1.5 seconds
```

The `write` command reveals text character by character.

### Math / LaTeX

```
draw math "a^{2} + b^{2} = c^{2}" at center color yellow size 72
draw math "E = mc^{2}" at bottom color cyan size 64
draw math "\frac{1}{2}bh" at center color white size 48
draw math "\pi \approx 3.14" at top color green size 56
```

**Supported syntax:**
| Syntax | Result |
|--------|--------|
| `^{2}` | superscript: ² |
| `_{n}` | subscript: ₙ |
| `\frac{a}{b}` | fraction: a/b |
| `\pi \alpha \beta \gamma \theta \omega` | Greek letters |
| `\sum \infinity \sqrt \times \div \pm` | Math operators |
| `\neq \leq \geq \approx` | Relations |
| `\rightarrow \leftarrow` | Arrows |

### Waves & Graphs

```
draw a wave at center with size 600 amplitude 80 frequency 3 color cyan thickness 3
draw a grid at center with size 400 color grey thickness 1
```

### Function Plotting *(adapted from MathLikeAnim-rs)*

```
draw axes at center with size 600 color grey thickness 2
plot "sin({x})" from -5 to 5 color cyan thickness 3 over 2 seconds
plot "{x}^2 / 5" from -5 to 5 color yellow thickness 3 over 2 seconds
```

Plots animate progressively (curves draw left to right). Axes auto-generate tick marks and grid lines.

### Animations

```
fade in the circle over 1 second
fade out everything over 1.5 seconds
move the circle to right over 2 seconds
rotate the triangle by 90 over 1 second
scale the square to 2 x over 0.5 seconds
change color of the circle to red over 1 second
grow the circle to radius 200 over 1 second
shrink the circle to radius 50 over 0.5 seconds
highlight the triangle color yellow over 1 second
```

### Easing Functions

Add `easing <name>` to any animation:

```
fade in the circle over 1 second easing bounce
move the square to right over 2 seconds easing elastic
scale the triangle to 2 x over 1 second easing spring
```

| Easing | Effect |
|--------|--------|
| `smooth` | Smooth ease-in-out (default) |
| `bounce` | Bounces at the end |
| `elastic` | Springy overshoot |
| `spring` | Overshoots and settles |
| `back` | Slight overshoot |
| `ease-in` | Accelerating |
| `ease-out` | Decelerating |
| `linear` | Constant speed |
| `ease-in-quad` / `ease-out-quad` / `quad` | Quadratic |
| `ease-in-quart` / `ease-out-quart` / `quart` | Quartic |
| `ease-in-quint` / `ease-out-quint` / `quint` | Quintic |
| `ease-in-expo` / `ease-out-expo` / `expo` | Exponential |
| `ease-in-circ` / `ease-out-circ` / `circ` | Circular |
| `ease-in-sine` / `ease-out-sine` / `sine` | Sinusoidal |
| `ease-in-out-bounce` | Bounce both ends |
| `ease-in-out-elastic` | Elastic both ends |

### Timing

```
wait 1 second
wait 0.5 seconds
```

### Named Objects

```
draw a circle at center with radius 50 color red myBall
move myBall to right over 1 second
```

### Positions

| Position | Location |
|----------|----------|
| `center` | Middle of canvas |
| `top` | Top center |
| `bottom` | Bottom center |
| `left` | Left center |
| `right` | Right center |
| `(x, y)` | Exact pixel coordinates |

### Colors

**Named:** `red`, `green`, `blue`, `white`, `black`, `yellow`, `cyan`, `magenta`, `orange`, `purple`, `pink`, `grey`/`gray`

**Compound:** `dark blue`, `dark green`, `dark red`, `light blue`, `light green`, `light grey`/`light gray`, `dark grey`/`dark gray`

**Hex:** `color "#ff6b35"`

### Groups & Styles

```
style hero:
    color is cyan
    size is 64

draw text "Title" at top with style hero

group shapes:
    draw a circle at left color red
    draw a square at center color green
    draw a triangle at right color blue
```

---

## 🖥️ IDE

Run `cargo run -- ide` to launch the split-screen IDE at `http://localhost:3333`.

**Features:**
- Code editor (left) + live video preview (right)
- Auto-render on typing (1.5s debounce)
- Timeline scrubber with play/pause
- Frame-by-frame stepping
- Example buttons: Hello World, Math Demo, Shapes, Waves, Plotting
- Keyboard: `Ctrl+Enter` to render, `Space` to play/pause

---

## 🏗️ Architecture

```
src/
├── main.rs              # CLI entry point
├── cli/mod.rs           # CLI commands (run, render, new, ide)
├── lexer/
│   ├── mod.rs           # Tokenizer
│   └── token.rs         # Token definitions
├── parser/
│   ├── mod.rs           # Recursive descent parser
│   └── ast.rs           # AST node definitions
├── renderer/
│   ├── mod.rs           # Rendering orchestrator
│   ├── scene.rs         # Scene state management
│   ├── shapes.rs        # Shape drawing (circle, wave, grid...)
│   ├── plotting.rs      # Function plotting (exmex f(x) evaluation)
│   ├── animation.rs     # Easing functions & interpolation
│   ├── text.rs          # Font rendering (ab_glyph + Inter)
│   └── export.rs        # FFmpeg video export
├── ide/mod.rs           # Split-screen IDE web server
assets/
├── fonts/Inter.ttf      # Embedded font
└── ide.html             # IDE frontend
examples/
├── demo.sl              # Basic demo
└── math_demo.sl         # Math/LaTeX demo
```

**Pipeline:** `.sl` → Lexer → Parser → AST → Renderer (tiny-skia) → Frames → FFmpeg → MP4

---

## 📦 Dependencies

| Crate | Purpose |
|-------|---------|
| `clap` | CLI argument parsing |
| `tiny-skia` | 2D software rendering |
| `ab_glyph` | Font rendering |
| `image` | PNG frame export |
| `tiny_http` | IDE web server |
| `base64` | Frame encoding for IDE preview |
| `serde` / `serde_json` | JSON serialization |
| `exmex` | Math expression evaluation for f(x) plotting |
| `keyframe` | 30+ professional easing functions |
| `noise` | Perlin noise for procedural backgrounds |
| `colorgrad` | Gradient color utilities |

**External:** FFmpeg (optional, for MP4 export)

---

## 📄 File Extension

`.sl` — Slang animation files

---

## License

MIT
