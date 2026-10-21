' TEST: cpp
$CONSOLE:ONLY
' The handler chain in C++ (m4-handlers-logging-checking task 2.2, design D1): _NEWHANDLER keeps the active
' handler's number in error_handler_history and sets the new one, _LASTHANDLER takes the first number back, as
' the old compiler writes them (qb64pe.bas 10066-10068, 10106-10110); a SUB's _NEWHANDLER numbers its label
' among the main module's.
ON ERROR GOTO ha
ON ERROR GOTO _NEWHANDLER hb
ON ERROR GOTO _LASTHANDLER
ON ERROR GOTO 0
s
SYSTEM
ha:
RESUME NEXT
hb:
RESUME NEXT
hc:
RESUME NEXT

SUB s
    ON ERROR GOTO _NEWHANDLER hc
    ON ERROR GOTO _LASTHANDLER
END SUB
