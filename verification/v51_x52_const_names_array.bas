$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x52_const_names_array)
DIM a(3) AS LONG
CONST c = a + 1
PRINT c
SYSTEM
