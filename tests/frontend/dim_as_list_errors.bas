' TEST: check-fail
$CONSOLE:ONLY
' The type before the names (m2-defxxx-common-arrays task 3.3): the old compiler's errors (verification\v24_x227
' to x232, x237): a type suffix on a name of the list, also the list's own; an AS clause on one; a name declared
' again; no name; SHARED after the type. An unknown type is "not supported yet", as in DIM name AS type, and
' stands last (a marked declaration drops the real errors after it).
DIM AS LONG a%
DIM AS LONG b&
DIM AS LONG c, d AS INTEGER
DIM AS LONG e, f
DIM f AS LONG
DIM AS LONG
DIM AS LONG SHARED g
show
SYSTEM
SUB show
    STATIC AS LONG k%
    SHARED AS LONG m!
END SUB
DIM AS nosuch h
