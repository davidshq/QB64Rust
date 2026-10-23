$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x59_dim_dot_array)
DIM a . b(3) AS LONG
a.b(1) = 2
PRINT a . b(1); a.b(1); UBOUND(a.b)
SYSTEM
