$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (33_element_assign)
DIM a(2) AS LONG, b(2) AS LONG
a(0) = 4
b(1) = a() * 2
PRINT b(1)
SYSTEM
