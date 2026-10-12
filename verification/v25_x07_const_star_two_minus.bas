$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): CONST: two - after another operator are one +.
x = 3: s$ = "a"
CONST c = 2 * - -3: PRINT c
SYSTEM
