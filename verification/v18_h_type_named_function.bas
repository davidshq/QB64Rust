$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): a TYPE named like a built-in function (LEN), and members named like
' built-ins and keywords.
TYPE len
    left AS LONG
    print AS LONG
    x AS LONG
END TYPE
DIM p AS len
p.left = 1: p.x = 2
PRINT p.left; p.x; LEN("abc")
SYSTEM
