$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x45_const_names_const_below)
CONST a = b + 1
CONST b = 2
PRINT a; b
SYSTEM
