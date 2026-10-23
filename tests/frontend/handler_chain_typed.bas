' TEST: typed
$CONSOLE:ONLY
' The handler chain (m4-handlers-logging-checking task 2.1, design D1): ON ERROR GOTO label, _NEWHANDLER label in
' any letter case, _LASTHANDLER, and 0; _NEWHANDLER in a SUB and a FUNCTION names a label of the main module.
ON ERROR GOTO ha
ON ERROR GOTO _newhandler hb
ON ERROR GOTO _LastHandler
ON ERROR GOTO 0
s
PRINT f
SYSTEM
ha:
RESUME NEXT
hb:
RESUME NEXT

SUB s
    ON ERROR GOTO _NEWHANDLER hb
    ON ERROR GOTO _LASTHANDLER
END SUB

FUNCTION f
    ON ERROR GOTO _NEWHANDLER ha
    f = 1
    ON ERROR GOTO _LASTHANDLER
END FUNCTION
