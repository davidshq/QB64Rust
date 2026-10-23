$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (37_string_assign)
DIM s(2) AS STRING
s(0) = "abc"
x$ = s()
PRINT x$
SYSTEM
