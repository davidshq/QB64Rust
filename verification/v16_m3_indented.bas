$CONSOLE:ONLY
' Verification (m2-parser-breadth, M3): an indented line number, and a line number inside a SUB.
GOTO 10
PRINT "skipped"
    10 PRINT "indented ten"
s
SYSTEM
SUB s
GOTO 10
10 PRINT "ten in s"
END SUB
