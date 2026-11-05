$CONSOLE:ONLY
' Verification: BASIC PDS 7.1's ON LOCAL ERROR GOTO in the main module. QB64pe has no such form.
ON LOCAL ERROR GOTO h
PRINT "ok"
SYSTEM
h:
RESUME NEXT
