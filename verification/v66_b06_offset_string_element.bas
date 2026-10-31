$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _OFFSET of an element of an array of variable-length strings: the distance between two elements.
DIM s(3) AS STRING
PRINT _OFFSET(s(2)) - _OFFSET(s(1))
SYSTEM
