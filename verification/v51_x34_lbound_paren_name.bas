$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x34_lbound_paren_name)
DIM a(10) AS LONG
PRINT LBOUND((a))
SYSTEM
