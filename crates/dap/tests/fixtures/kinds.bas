$CONSOLE:ONLY
TYPE pt
    x AS INTEGER
    nm AS STRING * 4
END TYPE
DIM SHARED total AS LONG
DIM s AS STRING
DIM a(1 TO 3) AS SINGLE
DIM p AS pt
total = 7: s = "hi": a(2) = 1.5: p.x = 3: p.nm = "abcd"
CALL outer(5)
PRINT total
SYSTEM
SUB outer (k AS LONG)
    DIM y AS LONG
    y = k * 2
    CALL show(y)
END SUB
SUB show (n AS LONG)
    STATIC calls AS INTEGER
    DIM x AS SINGLE
    x = n + 0.5
    calls = calls + 1
    total = total + n
END SUB
