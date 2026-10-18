$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): one form: _CEIL beyond _INTEGER64
CONST c = _CEIL(1E+30)
PRINT "accepted:"; c
SYSTEM
