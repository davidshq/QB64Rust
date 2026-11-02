$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (SOMEDAY.md, the m4-upstream-rejections row on names that are no type): REDIM of a _DYNAMIC member array AS a name that is no type.
TYPE dy
    k AS LONG
    _DYNAMIC d(1 TO 3) AS LONG
END TYPE
DIM x AS dy
REDIM x.d(5) AS nosuch
PRINT UBOUND(x.d)
SYSTEM
