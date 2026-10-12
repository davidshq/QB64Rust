' TEST: ir
$CONSOLE:ONLY
' Precompiler flags (m2-console-screenhide task 6.1; verification/v26_x28_flag_explicit): with $CONSOLE:ONLY alone
' _CONSOLE_ is 2; OPTION _EXPLICIT on the last line sets _EXPLICIT_ and _EXPLICITARRAY_ for the lines above it;
' the flags nothing sets (_ASSERTS_, _DEBUG_, _SOCKETS_) are 0.
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
$IF _ASSERTS_ OR _DEBUG_ OR _SOCKETS_ THEN
    PRINT "another flag is set"
$ELSE
    PRINT "asserts, debug, sockets 0"
$END IF
$IF _CONSOLE_ = 2 THEN
    PRINT "console 2"
$ELSEIF _CONSOLE_ = 1 THEN
    PRINT "console 1"
$END IF
SYSTEM
OPTION _EXPLICIT
