$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (165_shell_function_hide)
DIM n AS LONG
n = SHELL(_HIDE "a")
SYSTEM
