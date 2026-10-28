$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 9, the same block of _MEMNEW(0) freed twice (errors 300 to 315
' are not trapped).
ON ERROR GOTO h
DIM n AS _MEM
n = _MEMNEW(0)
_MEMFREE n
PRINT "first freed"
_MEMFREE n
PRINT "second freed"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
