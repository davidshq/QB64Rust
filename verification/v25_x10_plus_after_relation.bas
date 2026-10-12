$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): a + after a relation.
x = 3: s$ = "a"
PRINT 2 < +x
SYSTEM
