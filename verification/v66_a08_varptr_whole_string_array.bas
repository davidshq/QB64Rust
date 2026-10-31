$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR of a whole array of variable-length strings.
DIM s(3) AS STRING
s(0) = "abc"
PRINT VARPTR(s())
SYSTEM
