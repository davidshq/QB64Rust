' TEST: typed
$CONSOLE:ONLY
' The type before the names (m2-defxxx-common-arrays task 3.3; verification\v24_i_dim_as, v24_x233): DIM, DIM
' SHARED, STATIC and SHARED with scalars and arrays in one list, every name declared as `name AS type` would be
' (the plain name typed, whatever the default type), _UNSIGNED and STRING * n types, a user type, a list that
' starts with an array
TYPE rec
    n AS LONG
END TYPE
DEFINT A-Z
DIM AS LONG a, b(3)
DIM AS _UNSIGNED _BYTE u1, u2
DIM AS STRING * 4 f1, f2(2)
DIM AS STRING s1
DIM AS rec r1, r2(2)
DIM SHARED AS DOUBLE g1, g2(1 TO 2)
DIM AS INTEGER c(1 TO 2), d(2, 2), e
a = 100000: b(3) = a& + u1~%% + e
f1 = s1 + f2(1)
r1.n = r2(2).n
g2(2) = g1
keep
SYSTEM
SUB keep
    STATIC AS LONG k1, k2
    SHARED AS LONG a
    k1 = k2 + a + g1
END SUB
