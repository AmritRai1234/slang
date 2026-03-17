# Hello Slang! - Demo Animation
# Render with: slang render examples/demo.sl

scene "Hello Slang"

set background to dark blue
set resolution to 1920x1080
set fps to 30

# Draw a circle and fade it in
draw a circle at center with radius 150 color cyan
fade in the circle over 1.5 seconds

wait 0.5 seconds

# Write some text
write "Hello, Slang!" at top color white size 64 over 1.5 seconds

wait 1 second

# Move the circle to the right
move the circle to right over 1 second

wait 0.5 seconds

# Draw a square on the left
draw a square at left with size 120 color yellow
fade in the square over 1 second

# Change colors
change color of the circle to orange over 1 second

wait 1 second

# Fade everything out
fade out everything over 1.5 seconds
