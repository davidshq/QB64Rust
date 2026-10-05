$CONSOLE:ONLY
' Verification (m2-parser-breadth, M3): a line number before a SUB header inside another SUB.
s
SYSTEM
SUB s
PRINT "in s"
10 SUB t
END SUB
