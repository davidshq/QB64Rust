$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): one form: a numeric constant with $ inside a CONST
CONST n = 5
CONST c = n$ + ""
PRINT "accepted:"; c
SYSTEM
