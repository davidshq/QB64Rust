' QB 4.5 and PDS 7.1: which pixels a solid LINE lights, for every end point within 12 pixels.
' Found by the Wars of Napoleon port (its docs\devnotes.md, branch qb64rust, step 4): with the
' dotted style started at the same end (DIVERGENCES-QB45.md Q-024), some of the game's roads still
' had their dots one pixel to the side of DOSBox-X's.
' Each line is drawn from (50, 50) to (50 + dx, 50 + dy), then from the other end. A line of output
' is dx, dy, and for each draw the pixels along the longer axis: at each step from x (or y) = 50,
' the coordinate on the other axis, as a letter ("m" for 50, "l" for 49, "n" for 51, and so on).
' The second draw is "=" when it lit the same pixels as the first.
OPEN "OUT.TXT" FOR OUTPUT AS #1
SCREEN 12
FOR dx = -12 TO 12
	FOR dy = -12 TO 12
		CLS: LINE (50, 50)-(50 + dx, 50 + dy), 15: GOSUB scan: a$ = s$
		CLS: LINE (50 + dx, 50 + dy)-(50, 50), 15: GOSUB scan: b$ = s$
		IF a$ = b$ THEN b$ = "="
		PRINT #1, dx; dy; a$; " "; b$
	NEXT dy
NEXT dx
CLOSE #1
SYSTEM

scan:
s$ = ""
IF ABS(dx) >= ABS(dy) THEN
	st = SGN(dx): IF st = 0 THEN st = 1
	FOR x = 50 TO 50 + dx STEP st
		c$ = ""
		FOR y = 37 TO 63
			IF POINT(x, y) <> 0 THEN c$ = c$ + CHR$(109 + y - 50)
		NEXT y
		IF c$ = "" THEN c$ = "?"
		IF LEN(c$) > 1 THEN c$ = "(" + c$ + ")"
		s$ = s$ + c$
	NEXT x
ELSE
	st = SGN(dy)
	FOR y = 50 TO 50 + dy STEP st
		c$ = ""
		FOR x = 37 TO 63
			IF POINT(x, y) <> 0 THEN c$ = c$ + CHR$(109 + x - 50)
		NEXT x
		IF c$ = "" THEN c$ = "?"
		IF LEN(c$) > 1 THEN c$ = "(" + c$ + ")"
		s$ = s$ + c$
	NEXT y
END IF
RETURN
