$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): v58_e05 without the setting.
ON ERROR GOTO handler
10 ERROR 5
PRINT "after 5"
ON ERROR GOTO 0
DIM a(2)
i = 5
PRINT a(i)
PRINT "not reached"
SYSTEM
handler:
PRINT "error"; ERR; ERL; _ERRORLINE
RESUME NEXT
