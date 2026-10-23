$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, review): does the old compiler accept this, and what does it do? (39_varptr_whole)
DIM a(5 TO 7) AS LONG
PRINT VARPTR(a()) = VARPTR(a(5))
PRINT VARSEG(a()) = VARSEG(a(5))
SYSTEM
