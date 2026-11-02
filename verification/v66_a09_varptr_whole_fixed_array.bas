$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR of a whole array of fixed-length strings.
DIM f(3) AS STRING * 4
f(0) = "abcd"
PRINT VARPTR(f())
SYSTEM
