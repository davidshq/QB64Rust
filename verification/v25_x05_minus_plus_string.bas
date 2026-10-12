$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): -+ before a string.
x = 3: s$ = "a"
PRINT -+s$
SYSTEM
