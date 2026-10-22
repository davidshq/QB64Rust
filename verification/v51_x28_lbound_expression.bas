$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x28_lbound_expression)
DIM a(10) AS LONG
PRINT LBOUND(a + 1)
SYSTEM
