$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): SYSTEM n where n raises an error under ON ERROR (the check of qbevent before exit_code is stored).
ON ERROR GOTO handler
PRINT "x"
SYSTEM f
PRINT "not reached"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
FUNCTION f
    ERROR 5
    f = 9
END FUNCTION
