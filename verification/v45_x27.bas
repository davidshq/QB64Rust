$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): one form: SIN beyond 2^63
CONST c = SIN(1E+19)
PRINT "accepted:"; c
SYSTEM
