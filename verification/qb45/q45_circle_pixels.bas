' QB 4.5 and PDS 7.1: which pixels a small CIRCLE lights, with the screen's own aspect.
' Found by the Wars of Napoleon port (its TODO.md, branch qb64rust): the game draws each city
' with CIRCLE of radius 4 and 3 in SCREEN 12, and a charging unit with CIRCLE of radius 4 in
' SCREEN 9. QB64 lights other pixels at the top and bottom of those circles than DOSBox-X shows.
' For each screen and each radius from 1 to 10, the circle is drawn around (50, 50). The block
' that follows has one line per row from 50 - r - 1 to 50 + r + 1: "#" for a lit pixel, "." for
' one that is not, from x = 50 - r - 1 to 50 + r + 1. Then the same radius with PAINT inside it,
' as the game's city markers do: the count of lit pixels in that square.
' Last, the game's ellipses in SCREEN 12, with the same blocks: radius 18 with aspect .4 (the
' ships, shipicon in WON2.BM), radius 8 with aspect .4 (WON1.BM), and radius 70 with aspect .25
' (the "RAID COMMERCE" mark, WONMENU.BM), each around (100, 100) and from row 100 - r - 1.
OPEN "OUT.TXT" FOR OUTPUT AS #1
FOR m = 1 TO 2
	IF m = 1 THEN SCREEN 12: s$ = "SCREEN 12" ELSE SCREEN 9: s$ = "SCREEN 9"
	FOR r = 1 TO 10
		CLS
		CIRCLE (50, 50), r, 15
		PRINT #1, s$; " radius"; r
		FOR y = 50 - r - 1 TO 50 + r + 1
			l$ = ""
			FOR x = 50 - r - 1 TO 50 + r + 1
				IF POINT(x, y) <> 0 THEN l$ = l$ + "#" ELSE l$ = l$ + "."
			NEXT x
			PRINT #1, l$
		NEXT y
		PAINT (50, 50), 15, 15
		n = 0
		FOR y = 50 - r - 1 TO 50 + r + 1
			FOR x = 50 - r - 1 TO 50 + r + 1
				IF POINT(x, y) <> 0 THEN n = n + 1
			NEXT x
		NEXT y
		PRINT #1, "painted"; n
	NEXT r
NEXT m
SCREEN 12
FOR k = 1 TO 3
	SELECT CASE k
		CASE 1: r = 18: a! = .4: a$ = "aspect .4"
		CASE 2: r = 8: a! = .4: a$ = "aspect .4"
		CASE 3: r = 70: a! = .25: a$ = "aspect .25"
	END SELECT
	CLS
	CIRCLE (100, 100), r, 15, , , a!
	PRINT #1, "SCREEN 12 radius"; r; a$
	FOR y = 100 - r - 1 TO 100 + r + 1
		l$ = ""
		FOR x = 100 - r - 1 TO 100 + r + 1
			IF POINT(x, y) <> 0 THEN l$ = l$ + "#" ELSE l$ = l$ + "."
		NEXT x
		PRINT #1, l$
	NEXT y
	PAINT (100, 100), 15, 15
	n = 0
	FOR y = 100 - r - 1 TO 100 + r + 1
		FOR x = 100 - r - 1 TO 100 + r + 1
			IF POINT(x, y) <> 0 THEN n = n + 1
		NEXT x
	NEXT y
	PRINT #1, "painted"; n
NEXT k
CLOSE #1
SYSTEM
