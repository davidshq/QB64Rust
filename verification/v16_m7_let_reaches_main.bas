$CONSOLE:ONLY
' Verification (m2-parser-breadth, M7): a $LET in an included file reaches the main file.
'$INCLUDE:'v16_inc/let.bi'
$IF FROMINC = 1 THEN
PRINT "FROMINC = 1"
$ELSE
PRINT "FROMINC not set"
$END IF
SYSTEM
