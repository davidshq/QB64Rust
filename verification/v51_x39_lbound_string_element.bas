$CONSOLE:ONLY
' Verification (m4-upstream-rejections, task 1.1): does the old compiler accept this? (x39_lbound_string_element)
DIM s(10) AS STRING
PRINT LBOUND(s(5))
SYSTEM
