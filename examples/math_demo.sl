# Math Demo - Pythagorean Theorem
# Shows off LaTeX math rendering in Slang

scene "Pythagorean Theorem"

set background to dark blue
set resolution to 1920x1080
set fps to 30

# Title
write "The Pythagorean Theorem" at top color white size 64 over 1.5 seconds

wait 1 second

# Draw a right triangle
draw a triangle at center with size 250 color cyan
fade in the triangle over 1 second

wait 1 second

# Show the famous equation using math mode
draw math "a^{2} + b^{2} = c^{2}" at bottom color yellow size 72
fade in the math over 2 seconds

wait 2 seconds

# Show pi
draw math "\pi \approx 3.14159" at (960, 750) color white size 48
fade in the math over 1.5 seconds

wait 1 second

# Show a fraction
draw math "A = \frac{1}{2}bh" at (960, 850) color green size 48
fade in the math over 1.5 seconds

wait 2 seconds

fade out everything over 1 second

next scene

scene "Euler's Identity"

set background to dark blue

write "The Most Beautiful Equation" at top color white size 56 over 1 second

wait 1 second

draw math "e^{i\pi} + 1 = 0" at center color cyan size 96
fade in the math over 2 seconds

wait 3 seconds

fade out everything over 1.5 seconds
