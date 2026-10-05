$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): TYPE field forms: name AS type, AS type name list, element arrays, fixed strings, _UNSIGNED, nested TYPE, comments.
TYPE inner
    v AS LONG
END TYPE
TYPE outer 
    ' a comment
    a AS INTEGER
    AS LONG b, c
    s AS STRING * 4
    AS STRING * 2 t
    u AS _UNSIGNED _BYTE
    n AS inner

    arr(1 TO 3) AS LONG
END TYPE
DIM o AS outer
o.n.v = 7: o.c = 3: o.t = "xy"
PRINT o.n.v; o.c; o.t
SYSTEM
