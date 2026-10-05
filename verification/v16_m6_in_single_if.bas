$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): $LET in a single-line IF.
IF 0 THEN $LET Z = 1
$IF Z = 1 THEN
PRINT "Z = 1 (set inside IF 0 THEN)"
$ELSE
PRINT "Z not 1"
$END IF
SYSTEM
