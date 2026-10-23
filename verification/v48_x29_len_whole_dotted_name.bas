$CONSOLE:ONLY
' Verification (m4-names-arrays, review): LEN of a whole array whose name has a dot and is no TYPE member.
DIM my.arr(3) AS LONG, b.c(1 TO 2) AS DOUBLE
REDIM d.y(4) AS INTEGER
PRINT LEN(my.arr()); LEN(b.c()); LEN(d.y())
SYSTEM
