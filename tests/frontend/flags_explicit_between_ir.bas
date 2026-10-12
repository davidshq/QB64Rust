' TEST: ir
$CONSOLE:ONLY
' Precompiler flags (m2-console-screenhide task 1.4; verification/v26_x51_flag_explicit_between): _EXPLICIT_
' between two OPTION _EXPLICIT lines is 1. An option changes no value on its line; the second round starts with 1.
OPTION _EXPLICIT
$IF _EXPLICIT_ THEN
    PRINT "between: explicit 1"
$ELSE
    PRINT "between: explicit 0"
$END IF
OPTION _EXPLICIT
SYSTEM
