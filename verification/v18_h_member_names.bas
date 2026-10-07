$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): members named like built-in functions, statements and keywords.
TYPE t
    left AS LONG
    len AS LONG
    color AS LONG
    name AS LONG
    print AS LONG
    x AS LONG
END TYPE
DIM p AS t
p.left = 1: p.len = 2: p.color = 3: p.name = 4: p.print = 5: p.x = 6
PRINT p.left; p.len; p.color; p.name; p.print; p.x; LEN("abc")
SYSTEM
