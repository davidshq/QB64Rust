$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): an inactive IF header inside $IF, then an active one after $END IF.
x = 1
$IF LINUX THEN
IF x = 2 THEN
$END IF
IF x = 1 THEN
    PRINT "inside"
END IF
SYSTEM
