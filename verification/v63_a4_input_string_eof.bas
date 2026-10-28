$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 2, INPUT$(1) with no file number at the end of the input (stdin the
' null device) under a handler.
ON ERROR GOTO h
DIM s AS STRING
s = "old"
s = INPUT$(1)
PRINT "<"; s; ">"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
