$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (28_assign_operand)
DIM a(2) AS LONG
a(0) = 4
x = a() + 1
PRINT x
SYSTEM
