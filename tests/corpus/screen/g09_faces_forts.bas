' Cut from Civil War Strategy (tools/screen/cut_game.py): BLOAD with a file name in a string, eight pictures one after the other into the same two arrays: the faces SUB battle loads and the forts SUB menu loads, each with the game's own three lines, put in two rows.
DEFINT A-Z
DIM SHARED graft(1 TO 1564)
DIM SHARED graphic(1 TO 1564)
SCREEN 12
FOR k = 1 TO 5
IF k < 4 THEN t$ = "fort" + LTRIM$(STR$(k - 1)) + ".vga"
IF k < 4 THEN
					DEF SEG = VARSEG(graft(1))
					BLOAD t$, VARPTR(graft(1))
					DEF SEG
PUT (100 * k - 80, 150), graft, PSET
END IF
t$ = "face" + LTRIM$(STR$(k)) + ".vga"
		DEF SEG = VARSEG(graphic(1))
		BLOAD t$, VARPTR(graphic(1))
		DEF SEG
PUT (100 * k - 80, 20), graphic, PSET
NEXT k
SYSTEM
