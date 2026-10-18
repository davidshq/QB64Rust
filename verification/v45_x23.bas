$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): one form: a string constant with % in a string expression
CONST s = "ab"
CONST c = s% + "x"
PRINT "accepted:"; c
SYSTEM
