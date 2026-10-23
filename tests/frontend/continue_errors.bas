' TEST: check-fail
$CONSOLE:ONLY
' _CONTINUE where no loop of its body is open, and with something after it (m4-devices-continue-paths task 2.2,
' design D1; verification\v47_x01-x03): outside any loop, after END of a loop, inside SELECT and IF with no loop
' around them, in a SUB whose only loop is around its call, with an argument. Each is an error that is not "not
' supported yet"
DIM n AS LONG
_CONTINUE
FOR n = 1 TO 2
NEXT
_CONTINUE
SELECT CASE n
    CASE 1: _CONTINUE
END SELECT
IF n THEN _CONTINUE
FOR n = 1 TO 2
    t
    _CONTINUE 1
NEXT
SUB t
    _CONTINUE
END SUB
