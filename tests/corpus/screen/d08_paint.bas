' PAINT in each form: a fill colour, a border colour, both left out, STEP, a tile string, a background string.
DIM t AS STRING, c AS LONG
SCREEN 12
CIRCLE (60, 60), 40, 4
PAINT (60, 60), 2, 4
LINE (120, 20)-(200, 100), 5, B
c = 2
PAINT (160, 60), c + 1, c + 3
LINE (220, 20)-(300, 100), , B
PAINT (260, 60)
LINE (320, 20)-(400, 100), 6, B
PAINT (360, 60), , 6
LINE (420, 20)-(500, 100), 7, B
PSET (450, 60), 0
PAINT STEP(10, 0), 9, 7
LINE (520, 20)-(600, 100), 9, B
PSET (550, 60), 0
PAINT STEP(10, 0), 9
LINE (20, 120)-(100, 200), 8, B
PAINT (60, 160), CHR$(170) + CHR$(85), 8
t = CHR$(255) + CHR$(0) + CHR$(0) + CHR$(255)
LINE (120, 120)-(200, 200), 10, B
PAINT (160, 160), t, 10
LINE (220, 120)-(300, 200), 11, B
PAINT (260, 160), 12, 11
PAINT (260, 160), t, 11, CHR$(255)
LINE (320, 120)-(400, 200), 13, B
PSET (350, 160), 0
PAINT STEP(10, 0), t + CHR$(60), 13
LINE (420, 120)-(500, 200), 14, B
PAINT (460, 160), 14
SYSTEM
