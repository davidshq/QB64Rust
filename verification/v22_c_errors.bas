$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Data"): out of data (error 4) and the state after RESUME NEXT; which
' targets of a READ are stored after one of them raises, and where the next READ starts; RESUME reading again.
ON ERROR GOTO h
DIM a AS LONG, b AS LONG, c AS LONG, d AS LONG, y AS _BYTE, x(3) AS LONG, s AS STRING
DATA 1, 2

PRINT "four targets, two items": RESTORE dlast
a = -7: b = -7: c = -7: d = -7
READ a, b, c, d
PRINT a; b; c; d
PRINT "again": c = -7: READ c: PRINT c
PRINT "a string when out of data": s = "old": READ s: PRINT "["; s; "]"
PRINT "a string after a number when out of data": s = "old": c = -7: READ c, s: PRINT c; "["; s; "]"
PRINT "after RESTORE": RESTORE: READ c: PRINT c

PRINT "an item too large in the middle": RESTORE dmid
a = -7: y = -7: b = -7
READ a, y, b
PRINT a; y; b
READ c: PRINT "next item:"; c

PRINT "no number in the middle": RESTORE dnonum
a = -7: b = -7: c = -7
READ a, b, c
PRINT a; b; c
READ c: PRINT "next item:"; c

PRINT "an element out of range in the middle": RESTORE dmid2
a = -7: b = -7: k = 9
READ a, x(k), b
PRINT a; b
READ c: PRINT "next item:"; c

PRINT "RESUME reads again": RESTORE dnonum
tries = 0: again = -1
READ a, b
again = 0
PRINT a; b; "tries"; tries
READ c: PRINT "next item:"; c
SYSTEM

dmid:
DATA 10, 300, 30, 40
dnonum:
DATA 11, zz, 33, 44
dmid2:
DATA 12, 22, 32, 42
h:
PRINT "  error"; ERR; "line"; _ERRORLINE
IF again THEN
    tries = tries + 1
    IF tries < 3 THEN RESUME
END IF
RESUME NEXT
dlast:
DATA 51, 52
