' Cut from Civil War Strategy (tools/screen/cut_game.py): graphics GET and PUT through a SUB on a shared array: the game's snapshot takes the screen under a place and puts it back, here over a drawing that was changed in between, and at a second place.
DEFINT A-Z
DIM SHARED image(300)
SCREEN 12
LINE (80, 80)-(120, 120), 9, BF
CIRCLE (100, 100), 8, 14
CALL snapshot(100, 100, 0)
LINE (60, 60)-(140, 140), 4, BF
CALL snapshot(100, 100, 1)
CALL snapshot(300, 200, 1)
SYSTEM

SUB snapshot (x, y, flag)
IF flag = 0 THEN GET (x - 10, y - 10)-(x + 10, y + 10), image
IF flag = 1 THEN PUT (x - 10, y - 10), image, PSET
END SUB
