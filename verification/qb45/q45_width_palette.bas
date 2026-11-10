' QB 4.5 and PDS 7.1: does WIDTH , 60 in SCREEN 12 keep a color set by PALETTE?
' Found by the Wars of Napoleon port (its TODO.md, branch qb64rust): the map sets entry 2 to a
' darker green with PALETTE, then the City Report starts with WIDTH , 60. DOSBox-X showed the
' report's green rows in that darker green; QB64 shows them in the default 00AA00.
' QB 4.5 cannot read a palette entry back, so each line reads the VGA's color registers
' (port &H3C7 to choose the entry, then three reads of &H3C9: red, green, blue, 0 to 63).
' A line is what was just done, then entries 1, 2 and 3 as red, green, blue.
OPEN "OUT.TXT" FOR OUTPUT AS #1
SCREEN 12
GOSUB dump: PRINT #1, "SCREEN 12"; d$
PALETTE 2, 65536 * 0 + 256 * 34 + 0
GOSUB dump: PRINT #1, "PALETTE 2"; d$
WIDTH , 60
GOSUB dump: PRINT #1, "WIDTH , 60"; d$
PALETTE 2, 65536 * 0 + 256 * 34 + 0
WIDTH , 30
GOSUB dump: PRINT #1, "WIDTH , 30"; d$
PALETTE 2, 65536 * 0 + 256 * 34 + 0
CLS
GOSUB dump: PRINT #1, "CLS"; d$
SCREEN 12
GOSUB dump: PRINT #1, "SCREEN 12 again"; d$
PALETTE 3, 65536 * 20 + 256 * 0 + 0
SCREEN 0
SCREEN 12
GOSUB dump: PRINT #1, "SCREEN 0, SCREEN 12"; d$
CLOSE #1
SYSTEM

dump:
d$ = ""
FOR e = 1 TO 3
	OUT &H3C7, e
	r = INP(&H3C9): g = INP(&H3C9): b = INP(&H3C9)
	d$ = d$ + " |" + STR$(r) + STR$(g) + STR$(b)
NEXT e
RETURN
