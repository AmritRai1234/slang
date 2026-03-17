# Algebra — The Language of Math
# 8-minute educational video
# Topics: Variables, Equations, PEMDAS, Linear Equations, Quadratics, Graphing

# ============================================================
# SCENE 1: Title Card (0:00 - 0:15)
# ============================================================
scene "Title"

set background to gradient dark blue purple
set fps to 30

write "Algebra" at center color white size 72 over 2 seconds

wait 1 second

write "The Language of Mathematics" at bottom color cyan size 32 over 1.5 seconds

wait 2 seconds

fade out everything over 1.5 seconds

# ============================================================
# SCENE 2: What is Algebra? (0:15 - 1:00)
# ============================================================
scene "What is Algebra"

set background to gradient dark blue purple
set fps to 30

write "What is Algebra?" at top color yellow size 56 over 1 second

wait 1 second

write "Algebra is about finding unknown values" at center color white size 36 over 2 seconds

wait 2 seconds

fade out the text over 1 second

write "We use letters to represent unknowns" at center color white size 36 over 1.5 seconds

wait 1 second

draw math "x = ?" at center color cyan size 72

grow in the math over 1 second

wait 2 seconds

indicate the math over 0.8 seconds

wait 1.5 seconds

fade out everything over 1.5 seconds

# ============================================================
# SCENE 3: Variables (1:00 - 2:15)
# ============================================================
scene "Variables"

set background to gradient dark blue purple
set fps to 30

write "Variables" at top color yellow size 56 over 1 second

wait 1 second

write "A variable is a letter that stands for a number" at center color white size 32 over 2 seconds

wait 2 seconds

fade out the text over 0.8 seconds

draw math "x = 5" at center color cyan size 64

spin in the math over 1.5 seconds

wait 2 seconds

fade out the math over 0.8 seconds

draw math "y = 10" at center color green size 64

grow in the math over 1 second

wait 2 seconds

fade out the math over 0.8 seconds

write "We can add variables together" at center color white size 32 over 1.5 seconds

wait 1.5 seconds

fade out the text over 0.5 seconds

draw math "x + y = 5 + 10 = 15" at center color yellow size 56

grow in the math over 1.5 seconds

wait 3 seconds

circumscribe the math color cyan over 1 second

wait 2 seconds

fade out everything over 1.5 seconds

# ============================================================
# SCENE 4: Solving Equations (2:15 - 3:45)
# ============================================================
scene "Solving Equations"

set background to gradient dark blue purple
set fps to 30

write "Solving Equations" at top color yellow size 56 over 1 second

wait 1 second

write "Goal: Get the variable alone on one side" at center color white size 32 over 2 seconds

wait 2 seconds

fade out the text over 0.8 seconds

write "Example:" at top color grey size 28 over 0.5 seconds

draw math "x + 3 = 7" at center color cyan size 64

grow in the math over 1 second

wait 2 seconds

indicate the math over 0.8 seconds

wait 1 second

fade out the math over 0.8 seconds

write "Step 1: Subtract 3 from both sides" at center color white size 28 over 1.5 seconds

wait 1.5 seconds

fade out the text over 0.5 seconds

draw math "x + 3 - 3 = 7 - 3" at center color yellow size 56

grow in the math over 1 second

wait 2 seconds

fade out the math over 0.8 seconds

draw math "x = 4" at center color green size 72

spin in the math over 1.5 seconds

wait 1 second

circumscribe the math color yellow over 1 second

wait 3 seconds

fade out everything over 1.5 seconds

# ============================================================
# SCENE 5: Order of Operations — PEMDAS (3:45 - 5:00)
# ============================================================
scene "PEMDAS"

set background to gradient dark blue purple
set fps to 30

write "Order of Operations" at top color yellow size 56 over 1 second

wait 1 second

write "PEMDAS" at center color cyan size 72 over 1 second

wait 1 second

wiggle the text over 0.6 seconds

wait 1 second

fade out the text over 0.5 seconds

write "P — Parentheses" at center color white size 36 over 1 second

wait 1.5 seconds

fade out the text over 0.3 seconds

write "E — Exponents" at center color white size 36 over 1 second

wait 1.5 seconds

fade out the text over 0.3 seconds

write "M D — Multiply and Divide" at center color white size 36 over 1 second

wait 1.5 seconds

fade out the text over 0.3 seconds

write "A S — Add and Subtract" at center color white size 36 over 1 second

wait 1.5 seconds

fade out the text over 0.5 seconds

write "Example:" at top color grey size 28 over 0.5 seconds

draw math "2 + 3 * 4" at center color cyan size 64

grow in the math over 1 second

wait 1.5 seconds

fade out the math over 0.5 seconds

write "Multiply first: 3 * 4 = 12" at center color white size 32 over 1 second

wait 1.5 seconds

fade out the text over 0.3 seconds

draw math "2 + 12 = 14" at center color green size 64

spin in the math over 1 second

wait 1 second

indicate the math over 0.8 seconds

wait 2 seconds

fade out everything over 1.5 seconds

# ============================================================
# SCENE 6: Linear Equations (5:00 - 6:15)
# ============================================================
scene "Linear Equations"

set background to gradient dark blue purple
set fps to 30

write "Linear Equations" at top color yellow size 56 over 1 second

wait 1 second

draw math "y = mx + b" at center color cyan size 72

grow in the math over 1.5 seconds

wait 1 second

circumscribe the math color yellow over 1 second

wait 1.5 seconds

fade out the math over 0.8 seconds

write "m = slope (how steep the line is)" at center color white size 32 over 1.5 seconds

wait 2 seconds

fade out the text over 0.5 seconds

write "b = y-intercept (where it crosses y-axis)" at center color white size 32 over 1.5 seconds

wait 2 seconds

fade out the text over 0.5 seconds

write "Example:" at top color grey size 28 over 0.5 seconds

draw math "y = 2x + 1" at center color cyan size 64

grow in the math over 1 second

wait 1 second

fade out the math over 0.5 seconds

write "slope = 2, crosses y-axis at 1" at center color green size 32 over 1.5 seconds

wait 2 seconds

fade out the text over 0.5 seconds

draw axes at center with size 400 color grey thickness 2

fade in the axes over 1 second

wait 1 second

plot "2 * {x} + 1" from -4 to 4 color cyan thickness 3 over 2 seconds

wait 3 seconds

fade out everything over 1.5 seconds

# ============================================================
# SCENE 7: Quadratic Formula (6:15 - 7:30)
# ============================================================
scene "Quadratic Formula"

set background to gradient dark blue purple
set fps to 30

write "The Quadratic Formula" at top color yellow size 56 over 1 second

wait 1 second

write "For any equation in the form:" at center color white size 28 over 1 second

wait 1 second

fade out the text over 0.3 seconds

draw math "ax^{2} + bx + c = 0" at center color cyan size 56

grow in the math over 1.5 seconds

wait 2 seconds

fade out the math over 0.8 seconds

write "The solution is:" at center color white size 28 over 1 second

wait 1 second

fade out the text over 0.3 seconds

draw math "x = (-b +/- sqrt(b^{2} - 4ac)) / 2a" at center color yellow size 48

spin in the math over 2 seconds

wait 1 second

circumscribe the math color cyan over 1 second

wait 2 seconds

fade out the math over 0.8 seconds

write "Example:" at top color grey size 28 over 0.5 seconds

draw math "x^{2} - 5x + 6 = 0" at center color cyan size 56

grow in the math over 1 second

wait 1.5 seconds

fade out the math over 0.5 seconds

write "a = 1, b = -5, c = 6" at center color white size 32 over 1 second

wait 1.5 seconds

fade out the text over 0.3 seconds

draw math "x = 2 or x = 3" at center color green size 64

spin in the math over 1.5 seconds

wait 1 second

indicate the math over 0.8 seconds

wait 2.5 seconds

fade out everything over 1.5 seconds

# ============================================================
# SCENE 8: Outro (7:30 - 8:00)
# ============================================================
scene "Outro"

set background to gradient dark blue purple
set fps to 30

write "Algebra is Everywhere!" at center color yellow size 56 over 1.5 seconds

wait 1 second

wiggle the text over 0.6 seconds

wait 1 second

write "Science, Engineering, Finance, Games..." at bottom color cyan size 32 over 2 seconds

wait 2 seconds

draw a circle at center with radius 40 color cyan

grow in the circle over 1 second

wait 0.5 seconds

draw a square at left with size 60 color yellow

spin in the square over 1 second

wait 0.5 seconds

draw a triangle at right with size 80 color pink

spiral in the triangle over 1 second

wait 1 second

write "Keep Practicing!" at center color white size 64 over 1.5 seconds

wait 2 seconds

fade out everything over 2 seconds
