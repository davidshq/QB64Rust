$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): two - before a string at the start of an expression.
x = 3: s$ = "a"
s$ = - -s$: PRINT s$
SYSTEM
