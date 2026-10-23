' TEST: check-fail
$CONSOLE:ONLY
' The handler chain's errors (m4-handlers-logging-checking task 2.1, design D1; the old compiler's messages in
' verification\v50_x01, x02, x04, x14, x23, x24, x26): a label or 0 after _LASTHANDLER, _NEWHANDLER alone, 0
' after _NEWHANDLER, a label not defined, a label of a SUB, and another word before the label (a parse error).
ON ERROR GOTO _LASTHANDLER h
ON ERROR GOTO _LASTHANDLER 0
ON ERROR GOTO _NEWHANDLER
ON ERROR GOTO _NEWHANDLER 0
ON ERROR GOTO _NEWHANDLER nowhere
ON ERROR GOTO foo h
SYSTEM
h:
RESUME NEXT

SUB s
    ON ERROR GOTO _NEWHANDLER inner
    EXIT SUB
    inner:
END SUB
