$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 5, BSAVE of 1000 bytes from &HFFFF:&HFFF0, which reads 981 bytes
' past the end of the block: whether the file is written, and its length.
ON ERROR GOTO h
DEF SEG = &HFFFF
BSAVE "v63_b3.tmp", &HFFF0, 1000
DEF SEG
PRINT "exists:"; _FILEEXISTS("v63_b3.tmp")
IF _FILEEXISTS("v63_b3.tmp") THEN
    OPEN "v63_b3.tmp" FOR BINARY AS #1
    PRINT "length:"; LOF(1)
    CLOSE #1
    KILL "v63_b3.tmp"
END IF
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
