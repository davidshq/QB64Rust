$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 5, BLOAD of 1000 bytes to &HFFFF:&HFFF0, 19 bytes before the end
' of libqb's block of conventional memory (1114099 bytes), so 981 bytes land past it. Under a handler.
ON ERROR GOTO h
DEF SEG = 0
BSAVE "v63_b1.tmp", 0, 1000
DEF SEG = &HFFFF
BLOAD "v63_b1.tmp", &HFFF0
PRINT "after BLOAD"
DEF SEG
KILL "v63_b1.tmp"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
