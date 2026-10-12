$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): CONST: a + at the start.
x = 3: s$ = "a"
CONST c = +5: PRINT c
SYSTEM
