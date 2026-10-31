$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _OFFSET of a whole array of variable-length strings.
DIM s(3) AS STRING
PRINT _OFFSET(s()) <> 0
SYSTEM
