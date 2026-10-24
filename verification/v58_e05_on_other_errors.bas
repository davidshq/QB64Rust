$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON changes nothing for other errors: ERL and _ERRORLINE under a
' handler, then error 9 with no handler (its message, as without the setting, v58_e06).
$ERRORLOCATION:ON
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
