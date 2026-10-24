$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): _COMPILERVERSION$ (the version of the release), _ENVIRONCOUNT beside a count of
' ENVIRON$(i), _UPTIME as a comparison (it grows), LPOS of each printer and out of range under a handler.
ON ERROR GOTO handler
PRINT _COMPILERVERSION$
n = 0
DO
    n = n + 1
LOOP UNTIL ENVIRON$(n) = ""
PRINT n - 1 = _ENVIRONCOUNT; _ENVIRONCOUNT > 0
PRINT _UPTIME > 0; _UPTIME < 100000000
u# = _UPTIME: _DELAY .05: PRINT _UPTIME > u#; _UPTIME - u# < 10
PRINT LPOS(0); LPOS(1); LPOS(2); LPOS(3)
x = LPOS(4): PRINT x
x = LPOS(-1): PRINT x
x = LPOS(1.4): PRINT x
PRINT LEN(_COMPILERVERSION$) > 0
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT
