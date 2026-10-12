$CONSOLE:ONLY
' Verification (known bug "a unary plus is a parse error", 2026-10-11): a + after NOT.
x = 3: s$ = "a"
PRINT NOT +x
SYSTEM
