# Slang Syntax Cheat Sheet

## Basics
```
scene "Title"                              # Start a scene
set background to dark blue                # Settings
set resolution to 1920x1080
set fps to 30
wait 1 second                              # Pause
next scene                                 # Switch to next scene
```

## Backgrounds
```
set background to dark blue                # Solid color
set background to gradient blue purple     # Linear gradient (top→bottom)
set background to radial dark blue black   # Radial gradient (center→edge)
set background to noise dark blue          # Perlin noise texture
```

## Shapes
```
draw a circle at center with radius 100 color cyan
draw a square at left with size 80 color yellow
draw a rectangle at right with size 120 color green
draw a triangle at top with size 100 color pink
draw a line from (100,200) to (500,200) color white thickness 3
draw an arrow from (100,300) to (600,300) color red
draw a wave at center with size 600 amplitude 80 frequency 3 color cyan thickness 3
draw a grid at center with size 400 color grey
draw axes at center with size 600 color grey thickness 2
```

## Function Plotting
```
plot "sin({x})" from -5 to 5 color cyan thickness 3 over 2 seconds
plot "{x}^2 / 5" from -5 to 5 color yellow thickness 3 over 2 seconds
```

## Text & Math
```
draw text "Hello" at center color white size 48
write "Animated text" at top color white size 36 over 1.5 seconds

draw math "a^{2} + b^{2} = c^{2}" at bottom color yellow size 64
draw math "\frac{1}{2}bh" at center color white size 48
draw math "\pi \approx 3.14" at top color green size 56
```

## Animations
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

## Manim-Style Animations
```
grow in the circle over 1 second           # GrowFromCenter
spin in the square over 1.5 seconds        # SpinInFromNothing
draw in the square over 1 second           # DrawBorderThenFill
indicate the circle over 0.8 seconds       # Indicate (yellow pulse)
wiggle the circle over 0.6 seconds         # Wiggle (shake)
circumscribe the circle color yellow       # Circumscribe (highlight outline)
spiral in the triangle over 1.5 seconds    # SpiralIn
flash the circle color yellow              # Flash (expanding burst)
```

## Easing (30+ options)
```
fade in the circle over 1 second easing bounce
move the square to right over 2 seconds easing ease-out-expo
```

| Family | In | Out | InOut |
|--------|-----|------|-------|
| Sine | ease-in-sine | ease-out-sine | sine |
| Quad | ease-in-quad | ease-out-quad | quad |
| Cubic | ease-in / smooth | ease-out | ease-in-out / cubic |
| Quart | ease-in-quart | ease-out-quart | quart |
| Quint | ease-in-quint | ease-out-quint | quint |
| Expo | ease-in-expo | ease-out-expo | expo |
| Circ | ease-in-circ | ease-out-circ | circ |
| Back | ease-in-back | back | ease-in-out-back |
| Elastic | ease-in-elastic | elastic | ease-in-out-elastic |
| Bounce | ease-in-bounce | bounce | ease-in-out-bounce |
| Other | spring | linear | |

## Positions
`center` `top` `bottom` `left` `right` `(x, y)`

## Colors
`red` `green` `blue` `white` `black` `yellow` `cyan` `magenta` `orange` `purple` `pink` `grey`
`dark blue` `light green` `"#ff6b35"` `gradient cyan purple`

## Math Symbols
`^{2}` sup · `_{n}` sub · `\frac{a}{b}` fraction · `\pi \alpha \theta \omega` greek · `\sum \sqrt \times \approx \infinity` operators
