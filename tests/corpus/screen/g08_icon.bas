' Cut from Civil War Strategy (tools/screen/cut_game.py): BLOAD, PUT and a GET out of the picture just drawn: the game's icon and the capital marker its main module takes from it, then the marker put in a row as SUB showcity puts it on a city.
DEFINT A-Z
DIM SHARED graphic(1 TO 1564), Ncap(60)
SCREEN 12
	DEF SEG = VARSEG(graphic(1))
	BLOAD "cwsicon.vga", VARPTR(graphic(1))
	DEF SEG

	PUT (100, 100), graphic, PSET
	GET (101, 101)-(113, 113), Ncap
FOR i = 0 TO 5: PUT (300 + 20 * i, 200 + 5 * i), Ncap, PSET: NEXT i
SYSTEM
