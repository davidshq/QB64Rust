$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (27_member_whole)
TYPE tm
    m(3) AS LONG
END TYPE
DIM t AS tm
t.m(0) = 100
PRINT t.m()
SYSTEM
