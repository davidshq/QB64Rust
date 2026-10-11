' Cut from Civil War Strategy (tools/screen/cut_game.py): graphics GET and PUT, VIEW, CLS 1: a message box drawn and taken away again. TICK waits for a key, which the keys file gives.
DEFINT A-Z
DIM SHARED mtx$(21), tlx, tly, size
SCREEN 12
LINE (100, 100)-(540, 300), 2, BF
CIRCLE (320, 200), 90, 14
CALL image2("A message over the drawing", 4)
SYSTEM

SUB image2 (a$, s)
DIM g2(1 TO 8000)
mtx$(1) = a$
tlx = 32 - LEN(a$) \ 2
tly = 10: size = 1
CALL mxw(wide)
IF wide > 59 THEN wide = 59: a$ = LEFT$(a$, 59)
x = 8 * (tlx + wide)
y = 16 * (tly + 1)
	GET (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), g2
	VIEW (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3) + 4)
	CLS 1
	VIEW
	LINE (8 * tlx, 16 * tly - 1)-(8 * (tlx + wide + 1) + 15, 16 * (tly + 3) + 7), 0, BF
	LINE (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), s, BF
	LINE (8 * tlx - 4, 16 * tly - 3)-(8 * (tlx + wide + 1) + 7, 16 * (tly + 3)), 15, B
	COLOR 14
	LOCATE tly + 2, tlx + 2: PRINT a$
	CALL TICK(9)
	PUT (8 * tlx - 4, 16 * tly - 3), g2, PSET
END SUB

SUB mxw (wide)
	wide = LEN(mtx$(0)) + 1
	FOR i = 1 TO size
	x = LEN(mtx$(i))
	IF x > wide THEN wide = x
	NEXT i
END SUB

SUB TICK (sec!)
start! = TIMER
DO WHILE TIMER - start! < sec! AND INKEY$ = "": LOOP
END SUB
