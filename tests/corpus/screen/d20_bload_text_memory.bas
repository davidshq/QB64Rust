' Screen program (Civil War Battleset port, step 3): a text screen saved with BSAVE from &HB800 and
' loaded back with BLOAD into &HB800 in SCREEN 0, as the game's scrnload SUB shows its victory and high-score
' screens. The program writes a screen of characters in several colors, saves it, clears the screen and loads it
' back in a SUB. Then it compares each of the 2,000 cells with the saved file, through SCREEN(r, c),
' SCREEN(r, c, 1) and PEEK, and prints the counts on the last row. The state at exit is the loaded screen.
DEFINT A-Z
DECLARE SUB scrnload (t$)
SCREEN 0: WIDTH 80, 25
COLOR 7, 0: CLS
FOR r = 1 TO 24
	COLOR 1 + (r MOD 15), r MOD 8
	LOCATE r, 1
	FOR c = 1 TO 80
		PRINT CHR$(33 + (r * 7 + c) MOD 200);
	NEXT c
NEXT r
DEF SEG = &HB800
BSAVE "d20.scr", 0, 4000
DEF SEG
COLOR 7, 0: CLS
CALL scrnload("d20.scr")
OPEN "d20.scr" FOR BINARY AS #1
d$ = SPACE$(LOF(1)): GET #1, 1, d$: CLOSE #1
chars = 0: attrs = 0: peeks = 0
FOR r = 1 TO 25
	FOR c = 1 TO 80
		o = 8 + 2 * ((r - 1) * 80 + (c - 1))
		ch = ASC(MID$(d$, o, 1)): at = ASC(MID$(d$, o + 1, 1))
		IF SCREEN(r, c) = ch THEN chars = chars + 1
		IF SCREEN(r, c, 1) = at THEN attrs = attrs + 1
		DEF SEG = &HB800
		IF PEEK(o - 8) = ch AND PEEK(o - 7) = at THEN peeks = peeks + 1
		DEF SEG
	NEXT c
NEXT r
KILL "d20.scr"
COLOR 15, 0: LOCATE 25, 1
PRINT "bytes"; LEN(d$); " chars"; chars; " colors"; attrs; " peeks"; peeks;
SYSTEM

SUB scrnload (t$)
DEF SEG = &HB800
BLOAD t$, 0
DEF SEG
END SUB
