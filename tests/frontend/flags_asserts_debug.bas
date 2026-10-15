' TEST: check-ok
' The precompiler flags of $ASSERTS:CONSOLE and $DEBUG (m2-console-screenhide design D7;
' verification/v26_x52_flag_asserts_console, v26_x56_flag_asserts_running_value; m3-asserts-debug-zlib design D1,
' D2, verification/v33_d01, v33_d11): both are compiled now, and set their flags, so the rest of the program is read
' as the old compiler reads it: $ASSERTS:CONSOLE gives the program a console (_ECHO is no error),
' _CONSOLE_, _ASSERTS_, _DEBUG_ and _SOCKETS_ are 1 above and below their lines, and the OPTION _EXPLICIT inside
' $IF _ASSERTS_ = 0 below the line is met in no round (the variable that is not declared is no error). The
' branches that are not compiled each hold an error that would show.
$SCREENHIDE
$IF _CONSOLE_ = 1 AND _ASSERTS_ = 1 THEN
    _ECHO "above: console 1, asserts 1"
$ELSE
    x = "above"
$END IF
$IF _SOCKETS_ = 1 THEN
    _ECHO "above: sockets 1"
$ELSE
    x = "sockets"
$END IF
$ASSERTS:CONSOLE
$DEBUG
$IF _ASSERTS_ = 0 THEN
    OPTION _EXPLICIT
$END IF
$IF _DEBUG_ THEN
    _ECHO "debug 1"
$ELSE
    x = "debug"
$END IF
$IF _EXPLICIT_ THEN
    x = "explicit"
$END IF
notDeclared = 1
SYSTEM
