$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (36_string_concat)
DIM s(2) AS STRING
s(0) = "abc"
x$ = s() + "d"
PRINT x$
SYSTEM
