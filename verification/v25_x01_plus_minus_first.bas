$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): a + before a - at the start of an expression.
x = 3: s$ = "a"
PRINT +-x
SYSTEM
