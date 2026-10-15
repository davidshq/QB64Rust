$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): _INFLATE$ of bytes that are not deflated, under ON ERROR (lengths only: a first run printed
' 10 MB of zero bytes per string).
ON ERROR GOTO handler
PRINT "before"
s$ = _INFLATE$("not deflated")
PRINT LEN(s$); s$ = STRING$(LEN(s$), 0)
s$ = _INFLATE$("")
PRINT LEN(s$)
c$ = _DEFLATE$("hello world")
s$ = _INFLATE$(LEFT$(c$, 4))
PRINT LEN(s$); s$ = STRING$(LEN(s$), 0)
s$ = _INFLATE$(LEFT$(c$, 4), 5)
PRINT LEN(s$)
s$ = _INFLATE$("not deflated", 3)
PRINT LEN(s$)
s$ = _INFLATE$(c$, 3)
PRINT LEN(s$); "["; s$; "]"
PRINT "after"
SYSTEM
handler:
PRINT "handler"; ERR
RESUME NEXT
