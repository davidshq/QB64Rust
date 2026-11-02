$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-constant-evaluation row): SIN of a CONST whose value is infinite.
CONST i = 1F+4000 * 1F+4000
CONST c = SIN(i)
PRINT c
SYSTEM
