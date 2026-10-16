$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (56_copy_index_evaluations)
TYPE t
    n AS LONG
    v(1) _DYNAMIC AS LONG
    m AS LONG
END TYPE
TYPE u
    n AS LONG
    s AS STRING
    v(1) _DYNAMIC AS LONG
    w(1) _DYNAMIC AS STRING
END TYPE
REDIM a(2) AS t, b(2) AS t, c(2) AS u, d(2) AS u
b(1).v(1) = 5: d(1).s = "s": d(1).w(1) = "w"
a(one(1)) = b(one(2))
PRINT
c(one(1)) = d(one(2))
PRINT
PRINT a(1).v(1); c(1).s; c(1).w(1)
FUNCTION one& (k AS LONG)
    PRINT k;
    one& = 1
END FUNCTION
SYSTEM
