$CONSOLE:ONLY
$LET LEVEL = 2
$IF LEVEL = 1 THEN
    PRINT "Level 1: the short game."
$ELSEIF LEVEL = 2 THEN
    PRINT "Level 2: the long game."
$ELSE
    PRINT "Some other level."
$END IF
$IF TRACE = UNDEFINED THEN
    PRINT "TRACE is not defined, so the trace lines are left out."
$ELSE
    PRINT "Tracing."
$END IF
$IF 64BIT THEN
    PRINT "Compiled for 64 bits."
$END IF
END
