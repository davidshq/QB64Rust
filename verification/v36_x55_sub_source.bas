$CONSOLE:ONLY
' Verification (m3-whole-array-assign, by the review of the change): does the old compiler accept this? (55_sub_source)
DIM a(2) AS LONG
a() = s()
PRINT a(1)
SYSTEM
SUB s
END SUB
