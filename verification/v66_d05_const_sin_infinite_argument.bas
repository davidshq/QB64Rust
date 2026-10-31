$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-constant-evaluation row): SIN of an infinite value as the argument of another function in a CONST.
CONST c = ABS(SIN(1F+4000 * 1F+4000))
PRINT c
SYSTEM
