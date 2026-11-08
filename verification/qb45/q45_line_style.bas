' QB 4.5 and PDS 7.1: LINE with a style (the fourth argument), as Wars of Napoleon draws its
' roads (W.R. Hutsell, PDS 7.1; LINE ... , 0, , &H1111 from each end of a road, so every road is
' drawn twice, once in each direction). In the QB64 builds the game's dotted roads have other
' dots than in DOSBox-X (the game's docs\devnotes.md, branch qb64rust, step 4).
' For each case, the pixels along the line, one character per pixel ("." for 0, else the hex
' digit of the color), from the left (or the top) end.
OPEN "OUT.TXT" FOR OUTPUT AS #1
SCREEN 12

' One line in each direction, alone: does the pattern start at the first point given?
PRINT #1, "alone"
CLS: LINE (10, 10)-(42, 10), 15, , &H1111: GOSUB row
CLS: LINE (42, 10)-(10, 10), 15, , &H1111: GOSUB row
CLS: LINE (10, 10)-(42, 10), 15, , &HF0F0: GOSUB row
CLS: LINE (42, 10)-(10, 10), 15, , &HF0F0: GOSUB row
CLS: LINE (10, 10)-(42, 10), 15, , &HC000: GOSUB row
CLS: LINE (42, 10)-(10, 10), 15, , &HC000: GOSUB row
CLS: LINE (10, 10)-(10, 42), 15, , &HC000: GOSUB col
CLS: LINE (10, 42)-(10, 10), 15, , &HC000: GOSUB col
CLS: LINE (10, 10)-(42, 26), 15, , &HC000: GOSUB diag
CLS: LINE (42, 26)-(10, 10), 15, , &HC000: GOSUB diag
CLS: LINE (10, 26)-(42, 10), 15, , &HC000: GOSUB diag2
CLS: LINE (42, 10)-(10, 26), 15, , &HC000: GOSUB diag2

' Steeper than 45 degrees: the lit pixel of each row, as its color and column.
PRINT #1, "steep"
CLS: LINE (10, 10)-(26, 42), 15, , &HC000: GOSUB steep
CLS: LINE (26, 42)-(10, 10), 15, , &HC000: GOSUB steep
CLS: LINE (26, 10)-(10, 42), 15, , &HC000: GOSUB steep
CLS: LINE (10, 42)-(26, 10), 15, , &HC000: GOSUB steep
' Exactly 45 degrees.
CLS: LINE (10, 10)-(42, 42), 15, , &HC000: GOSUB steep
CLS: LINE (42, 42)-(10, 10), 15, , &HC000: GOSUB steep
CLS: LINE (42, 10)-(10, 42), 15, , &HC000: GOSUB steep
CLS: LINE (10, 42)-(42, 10), 15, , &HC000: GOSUB steep

' The game's case: the same road from each end, the second in another color.
PRINT #1, "both ends"
CLS: LINE (10, 10)-(42, 10), 15, , &H1111: LINE (42, 10)-(10, 10), 9, , &H1111: GOSUB row
CLS: LINE (10, 10)-(42, 26), 15, , &H1111: LINE (42, 26)-(10, 10), 9, , &H1111: GOSUB diag

' Does the pattern go on from one LINE to the next, or start again?
PRINT #1, "two lines"
CLS: LINE (10, 10)-(20, 10), 15, , &HC000: LINE (21, 10)-(42, 10), 15, , &HC000: GOSUB row
CLS: LINE (10, 10)-(20, 10), 15, , &HC000: LINE -(42, 10), 15, , &HC000: GOSUB row

CLOSE #1
SYSTEM

row:
a$ = "": FOR x = 10 TO 42: p = POINT(x, 10): IF p = 0 THEN a$ = a$ + "." ELSE a$ = a$ + HEX$(p)
NEXT x: PRINT #1, a$: RETURN

col:
a$ = "": FOR y = 10 TO 42: p = POINT(10, y): IF p = 0 THEN a$ = a$ + "." ELSE a$ = a$ + HEX$(p)
NEXT y: PRINT #1, a$: RETURN

' A line from (10, 10) to (42, 26): the lit pixel of each column, as its color and row.
diag:
a$ = ""
FOR x = 10 TO 42
	c$ = "."
	FOR y = 10 TO 26
		p = POINT(x, y): IF p <> 0 THEN c$ = HEX$(p) + LTRIM$(STR$(y - 10))
	NEXT y
	a$ = a$ + c$ + " "
NEXT x
PRINT #1, a$: RETURN

' Rows 10 to 42: the lit pixel of each row, as its color and column (from 10).
steep:
a$ = ""
FOR y = 10 TO 42
	c$ = "."
	FOR x = 10 TO 42
		p = POINT(x, y): IF p <> 0 THEN c$ = HEX$(p) + LTRIM$(STR$(x - 10))
	NEXT x
	a$ = a$ + c$ + " "
NEXT y
PRINT #1, a$: RETURN

' A line from (10, 26) to (42, 10).
diag2:
a$ = ""
FOR x = 10 TO 42
	c$ = "."
	FOR y = 10 TO 26
		p = POINT(x, y): IF p <> 0 THEN c$ = HEX$(p) + LTRIM$(STR$(26 - y))
	NEXT y
	a$ = a$ + c$ + " "
NEXT x
PRINT #1, a$: RETURN
