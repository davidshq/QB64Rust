$CONSOLE:ONLY
' Verification (SOMEDAY.md, the unknown-type row): REDIM of a member array that is not _DYNAMIC, AS a name that is no type.
TYPE fx
    k AS LONG
    d(1 TO 3) AS LONG
END TYPE
DIM x AS fx
REDIM x.d(3) AS nosuch
PRINT UBOUND(x.d)
SYSTEM
