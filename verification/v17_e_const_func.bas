$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a function in a CONST expression (the old compiler evaluates some).
CONST fl = LEN("abc"), fa = ABS(-2)
PRINT fl; fa
SYSTEM
