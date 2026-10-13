' TEST: ir
$CONSOLE:ONLY
' Precompiler flags (m2-console-screenhide task 6.1; verification/v26_x33_explicitarray_only): OPTION
' _EXPLICITARRAY alone sets _EXPLICITARRAY_ and leaves _EXPLICIT_ 0.
$IF _EXPLICIT_ THEN
    PRINT "explicit 1"
$ELSE
    PRINT "explicit 0"
$END IF
$IF _EXPLICITARRAY_ THEN
    PRINT "explicitarray 1"
$ELSE
    PRINT "explicitarray 0"
$END IF
SYSTEM
OPTION _EXPLICITARRAY
