$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _OFFSET of a member reached through an element of a member array whose elements are of a TYPE.
TYPE q
    k AS LONG
    j AS LONG
END TYPE
TYPE u
    a AS LONG
    n(1 TO 3) AS q
END TYPE
DIM x(2) AS u
PRINT _OFFSET(x(1).n(2).j) - _OFFSET(x(1))
SYSTEM
