$CONSOLE:ONLY
' Verification: a handler set by ON ERROR GOTO inside a SUB stays active after the SUB returns, unless the SUB
' turns it off with ON ERROR GOTO 0. BASIC PDS 7.1's ON LOCAL ERROR ends with its SUB, so a port that replaces it
' needs the ON ERROR GOTO 0 (docs\PORTING-PDS.md).
ON ERROR GOTO 0
s
PRINT "back in main"
ERROR 5
PRINT "after ERROR 5 in main"
t
PRINT "back in main again"
ERROR 6
PRINT "not reached"
SYSTEM
h:
PRINT "handler"; ERR
RESUME NEXT

SUB s
    ON ERROR GOTO h
    PRINT "in s"
END SUB

SUB t
    ON ERROR GOTO h
    ERROR 7
    PRINT "in t, after ERROR 7"
    ON ERROR GOTO 0
END SUB
