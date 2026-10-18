$CONSOLE:ONLY
' Verification (m3-constant-evaluation, task 1.2): one form: _RED of a 32-bit colour in mode 0
CONST c = _RED(&HFF112233, 0)
PRINT "accepted:"; c
SYSTEM
