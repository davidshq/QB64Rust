$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): LEN of a whole array and of a whole member array.
TYPE num_t
    a AS LONG
    b AS LONG
END TYPE
TYPE test
    a(1 TO 50) AS LONG
END TYPE
TYPE holder
    k AS INTEGER
    m(2) AS DOUBLE
END TYPE
ON ERROR GOTO h
PRINT "1. numbers and dimensions:";
DIM a(3 TO 7) AS INTEGER, b(1, 2) AS DOUBLE, c(4) AS _BYTE, d(2) AS _FLOAT, o(1) AS _OFFSET
PRINT LEN(a()); LEN(a(3)); LEN(b()); LEN(c()); LEN(d()); LEN(o())
PRINT "2. fixed-length strings and user types:";
DIM na(-3 TO 2) AS num_t, f(4) AS STRING * 3
PRINT LEN(na()); LEN(f())
PRINT "3. a dynamic array, before and after ERASE:";
n = 7
REDIM dy(3 TO n) AS INTEGER
PRINT LEN(dy());
ERASE dy
PRINT LEN(dy());
REDIM dy(1 TO 12) AS INTEGER
PRINT LEN(dy())
PRINT "4. a whole member array:";
DIM parent(1) AS test
PRINT LEN(parent(0).a()); LEN(parent()); LEN(parent(0).a(1))
DIM hv AS holder
PRINT LEN(hv.m()); LEN(hv)
PRINT "5. in an expression:";
DIM f4(3) AS LONG
PRINT LEN(f4()) / 4; LEN(f4()) \ 4; LEN(f4()) MOD 3; LEN(f4()) + LEN(a())
PRINT "6. an implicit array:";
im(2) = 1
PRINT LEN(im())
PRINT "7. an array parameter, a shared array:";
DIM SHARED sg(9) AS LONG
param a()
shared_len
PRINT "8. a suffixed array, a default-typed one:";
DIM l&(5), dflt(5)
PRINT LEN(l&()); LEN(dflt())
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB param (x() AS INTEGER)
    PRINT LEN(x());
END SUB

SUB shared_len
    PRINT LEN(sg())
END SUB
