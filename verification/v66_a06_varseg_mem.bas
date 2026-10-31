$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARSEG(m) of a _MEM variable.
DIM m AS _MEM
m = _MEMNEW(8)
PRINT VARSEG(m)
SYSTEM
