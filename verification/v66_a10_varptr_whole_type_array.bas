$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-get-put-bload row on addresses): VARPTR of a whole array of a TYPE of numbers.
TYPE pt
    x AS INTEGER
    y AS INTEGER
END TYPE
DIM t(3) AS pt
PRINT VARPTR(t())
SYSTEM
