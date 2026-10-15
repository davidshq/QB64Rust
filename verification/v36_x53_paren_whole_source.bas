$CONSOLE:ONLY
' Verification (m3-whole-array-assign, by the review of the change): does the old compiler accept this? (53_paren_whole_source)
DIM a(2) AS LONG, n AS LONG
n = (a())
PRINT n
SYSTEM
