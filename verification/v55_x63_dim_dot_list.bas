$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x63_dim_dot_list)
DIM a . b AS LONG, c . d AS INTEGER
a.b = 1: c.d = 2
PRINT a.b; c . d
SYSTEM
