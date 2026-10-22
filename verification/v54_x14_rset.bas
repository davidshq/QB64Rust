$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): does the old compiler accept this, and what does it do? (14_rset)
DIM a(2) AS STRING
a(0) = "abcd"
RSET a() = "x"
PRINT "["; a(0); "]"
SYSTEM
