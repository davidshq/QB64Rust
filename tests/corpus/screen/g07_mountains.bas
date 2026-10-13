' Cut from Civil War Strategy (tools/screen/cut_game.py): BLOAD of a picture into an array in conventional memory (DEF SEG, VARSEG, VARPTR) and graphics PUT with PSET: the game's mountains, loaded as its main module loads them and drawn as the first lines of SUB usa draw them.
DEFINT A-Z
DIM SHARED mtn(1 TO 1564)
	DEF SEG = VARSEG(mtn(1))
	BLOAD "mtn.vga", VARPTR(mtn(1))
	DEF SEG
SCREEN 12
LINE (1, 16)-(527, 440), 10, B
PAINT (200, 200), 2, 10
COLOR 10
'==========================  Mountains =================================
IF _FILEEXISTS("mtn.vga") THEN
	PUT (380, 30), mtn, PSET
	PUT (270, 200), mtn, PSET
	PUT (310, 160), mtn, PSET
	PUT (350, 120), mtn, PSET
	PUT (200, 185), mtn, PSET
	PUT (250, 130), mtn, PSET
	PUT (320, 80), mtn, PSET
	PUT (30, 150), mtn, PSET
	LINE (30, 150)-(70, 190), 2, BF
END IF
SYSTEM
