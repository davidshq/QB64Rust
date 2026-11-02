$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-names-arrays row): an array without AS beside one listed by COMMON name() AS type.
COMMON SHARED g() AS DOUBLE
DIM g(3)
g(1) = 1.5
PRINT g(1); UBOUND(g)
SYSTEM
