$CONSOLE:ONLY
' Verification: BASIC PDS 7.1's ON LOCAL ERROR GOTO in a SUB. QB64pe has no such form.
' The World War 2 game's port met it once (docs\PORTING-PDS.md).
s
PRINT "ok"
SYSTEM

SUB s
    ON LOCAL ERROR GOTO h
    PRINT "in s"
    EXIT SUB
    h:
    RESUME NEXT
END SUB
