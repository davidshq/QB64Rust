$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR of a member whose type is a TYPE, against VARPTR of its variable.
TYPE pt
    x AS INTEGER
    y AS INTEGER
END TYPE
TYPE box
    a AS LONG
    p AS pt
END TYPE
DIM o AS box
PRINT VARPTR(o.p) - VARPTR(o)
SYSTEM
