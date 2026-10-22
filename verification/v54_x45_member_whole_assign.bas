$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, review): does the old compiler accept this, and what does it do? (45_member_whole_assign)
TYPE tm
    m(3) AS LONG
END TYPE
DIM t AS tm, u AS tm
u.m(0) = 100
t.m&() = u.m&()
PRINT t.m(0)
SYSTEM
