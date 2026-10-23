$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x30_lbound_plain_element)
DIM a(10) AS LONG
PRINT LBOUND(a(5))
SYSTEM
