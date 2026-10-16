$CONSOLE:ONLY
' Verification (m3-whole-array-assign, by the review of the change): does the old compiler accept this? (54_paren_element_source)
DIM a(2) AS LONG, b(2) AS LONG
a() = (b(1))
PRINT a(1)
SYSTEM
