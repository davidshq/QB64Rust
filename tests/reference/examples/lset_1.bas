$CONSOLE:ONLY
s$ = "12345678"
LSET s$ = "ab"
PRINT "["; s$; "]"
RSET s$ = "ab"
PRINT "["; s$; "]"
LSET s$ = "much too long"
PRINT "["; s$; "]"
END
