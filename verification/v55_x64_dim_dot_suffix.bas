$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x64_dim_dot_suffix)
DIM a . b$
a.b$ = "s"
PRINT a . b$
SYSTEM
