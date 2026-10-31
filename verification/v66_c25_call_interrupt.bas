$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m4-names-implicit-arrays row): CALL INTERRUPT, the mouse reset of interrupt &H33.
TYPE RegType
    ax AS INTEGER
    bx AS INTEGER
    cx AS INTEGER
    dx AS INTEGER
    bp AS INTEGER
    si AS INTEGER
    di AS INTEGER
    flags AS INTEGER
END TYPE
DIM r AS RegType
r.ax = 0
CALL INTERRUPT(&H33, r, r)
PRINT r.ax; r.bx
SYSTEM
