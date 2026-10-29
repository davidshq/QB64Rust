' TEST: cpp
$CONSOLE:ONLY
' SWAP of places whose indexes call a FUNCTION (QB64PE-DIVERGENCES.md D-054): each place into a variable first, the
' first place first, so that the FUNCTIONs run left to right whatever the C++ compiler's order of arguments; for a
' TYPE the second place first, as its call takes them. With no FUNCTION in an index the call is the old compiler's.
TYPE rec
    n AS LONG
    v(3) AS LONG
END TYPE
DIM a(9) AS LONG, s(9) AS STRING, r(3) AS rec, k AS LONG
SWAP a(one(1)), a(one(2))
SWAP s(k), s(one(3))
SWAP r(one(1)).v(one(2)), r(one(3)).v(one(4))
SWAP r(one(1)), r(2)
SWAP a(k), a(k + 1)
SYSTEM
FUNCTION one& (n AS LONG)
    PRINT n;
    one& = 1
END FUNCTION
