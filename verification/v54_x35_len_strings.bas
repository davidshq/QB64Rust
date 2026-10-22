$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (35_len_strings)
DIM s(2) AS STRING
s(0) = "abc"
PRINT LEN(s())
SYSTEM
