# Waves & Easing Demo
# Shows off: sine waves, coordinate grids, easing, and math equations

scene "Wave Mathematics"

set background to dark blue
set fps to 30

# Title
write "Waves & Easing" at top color white size 56 over 1.5 seconds

wait 0.5 seconds

# Draw a coordinate grid
draw a grid at center with size 400 color grey thickness 1
fade in the grid over 0.5 seconds

wait 0.5 seconds

# Draw a sine wave on the grid
draw a wave at center with size 600 amplitude 80 frequency 3 color cyan thickness 3
fade in the wave over 1 second

wait 1 second

# Show the wave equation
draw math "y = A sin(\omega x)" at bottom color yellow size 48
fade in the math over 1.5 seconds

wait 1 second

# Bounce a circle in
draw a circle at (200, 340) with radius 30 color orange
fade in the circle over 0.5 seconds easing bounce

wait 0.5 seconds

# Spring a square in
draw a square at (700, 340) with size 50 color pink
fade in the square over 0.5 seconds easing spring

wait 1 second

# Move with elastic easing
move the circle to (500, 340) over 1.5 seconds easing elastic

wait 1 second

fade out everything over 1 second
