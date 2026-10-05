$CONSOLE:ONLY
' Verification (m2-parser-breadth, M3, group 6): a name and a colon after a colon: a label, or a SUB call?
PRINT "a": lab: PRINT "b"
GOTO lab2
PRINT "skipped"
PRINT "c": lab2: PRINT "d"
SYSTEM
