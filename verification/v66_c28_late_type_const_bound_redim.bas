$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m4-memory-records row): REDIM above its block of a _DYNAMIC member array whose bound in the block names a CONST.
CONST n = 3
DIM e AS late
REDIM e.v(1 TO 5) AS LONG
PRINT LBOUND(e.v); UBOUND(e.v)
SYSTEM

TYPE late
    k AS LONG
    _DYNAMIC v(1 TO n) AS LONG
END TYPE
