$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (10_swap_types_differ)
DIM a(2) AS LONG, b(2) AS INTEGER
a(0) = 1: b(0) = 2
SWAP a(), b()
PRINT a(0); b(0)
SYSTEM
