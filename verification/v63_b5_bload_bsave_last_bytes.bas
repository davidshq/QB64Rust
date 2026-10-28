$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 5, BSAVE and BLOAD of the block's last 19 bytes (&HFFFF:&HFFF0 to
' its end), which fit exactly: they work, and the bytes come back.
ON ERROR GOTO h
DIM i AS LONG
DEF SEG = &HFFFF
FOR i = 0 TO 18: POKE &HFFF0 + i, i + 1: NEXT
BSAVE "v63_b5.tmp", &HFFF0, 19
FOR i = 0 TO 18: POKE &HFFF0 + i, 0: NEXT
BLOAD "v63_b5.tmp", &HFFF0
FOR i = 0 TO 18: PRINT PEEK(&HFFF0 + i); : NEXT
PRINT
DEF SEG
KILL "v63_b5.tmp"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
