$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (18_print_two_dims_lower)
OPTION BASE 1
DIM a(2, 2) AS LONG
a(1, 1) = 11: a(2, 1) = 21
PRINT a()
SYSTEM
