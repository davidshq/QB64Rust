$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (29_assign_parenthesised)
DIM a(2) AS LONG
a(0) = 4
x = (a())
PRINT x
SYSTEM
