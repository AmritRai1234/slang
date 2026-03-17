# Variables and Loops Demo

scene "Programmable Slang"

set background to gradient dark blue purple
set fps to 30

# Variables work!
let count = 5

# Draw shapes in a loop
repeat count times with i
    draw a circle at center with radius 40 color cyan
    fade in the circle over 0.3 seconds

wait 0.5 seconds

# Use if/else with variables
let big = 1
if big > 0
    write "Slang is now PROGRAMMABLE!" at top color white size 48 over 1.5 seconds

wait 0.5 seconds

# Math expressions in let
let result = 10 + 5 * 2

# Another loop
repeat 3 times with j
    draw a square at center with radius 50 color yellow
    grow in the square over 0.4 seconds

wait 1 second

fade out everything over 1 second
