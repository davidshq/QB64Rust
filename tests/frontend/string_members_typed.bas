' TEST: typed
$CONSOLE:ONLY
' LEN of a whole member array of variable-length strings is 8 bytes an element, a STRING member being its
' pointer, and _MEM of a value of a TYPE with a STRING member is its bytes (m4-memory-records design D10;
' verification\v48_x11, v48_x30, v56_g1, v56_g2)
TYPE t1
    s(3) AS STRING
END TYPE
TYPE t2
    id AS INTEGER
    s(1 TO 2, 0 TO 2) AS STRING
    v AS LONG
END TYPE
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM x AS t1, y AS t2, ya(2) AS t2, a(3) AS ts, m AS _MEM
PRINT LEN(x.s()); LEN(y.s()); LEN(ya(1).s())
m = _MEM(a())
m = _MEM(a(2))
m = _MEMELEMENT(a(1))
SYSTEM
