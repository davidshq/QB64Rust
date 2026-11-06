$CONSOLE:ONLY
' v68 "Duplicate constant definition": a CONST repeated with the same value, as a number, a string, with a suffix,
' and as another expression of the same value.
CONST a = 5
CONST a = 5
CONST s$ = "x"
CONST s$ = "x"
CONST d% = 1
CONST d% = 1
CONST e = 6
CONST e = 2 + 4
CONST f = 2.5
CONST f = 2.5
PRINT a; s$; d%; e; f
SYSTEM
