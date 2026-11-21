$CONSOLE:ONLY
' Verification (QB64pe issue 559): one string of about 1.1 GB assigned seven times. The old string is freed each
' time, but the pool's place past its strings goes over 2 GB, where libqb doubles it in a uint32_t.
FOR i = 1 TO 7
    c$ = SPACE$(1123123123)
    PRINT i; LEN(c$)
NEXT
c$ = ""
PRINT "done"
SYSTEM
