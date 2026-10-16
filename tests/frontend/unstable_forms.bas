' TEST: check-fail
$CONSOLE:ONLY
' The forms of $UNSTABLE the old compiler rejects, and the markers where its flag does not reach (m3-typefields
' design D1, task 2.1; verification/v35_x10-x12, x14, x45). Each is an error of the program, none "not supported
' yet"; the messages are new. The forms it accepts are in unstable_forms_ok.bas.
$UNSTABLE:FOO
$UNSTABLE:
$UNSTABLE
$IF 0 THEN
    $UNSTABLE:TYPEFIELDS
$END IF
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE u
    w(1) AS LONG _STATIC
END TYPE
$UNSTABLE:TYPEFIELDS
SYSTEM
