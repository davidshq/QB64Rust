$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1): does the old compiler accept this? (115_mid_const_target)
CONST c = "abc"
MID$(c, 1) = "x"
SYSTEM
