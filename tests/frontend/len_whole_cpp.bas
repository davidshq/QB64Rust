' TEST: cpp
$CONSOLE:ONLY
' LEN of a whole array in the C++ (m4-names-arrays task 5.2, design D5; verification\v48_d_len_whole): the old
' compiler's "-5" size, the bytes of all elements from the descriptor ((int32), 0 while a dynamic array is not
' allocated), for a static, a dynamic, a fixed-length-string, a user-type and a two-dimensional array and an array
' parameter; a whole member array is a number.
TYPE num_t
    a AS LONG
    b AS LONG
END TYPE
TYPE test
    a(1 TO 50) AS LONG
END TYPE
DIM a(3 TO 7) AS INTEGER, f(4) AS STRING * 3, na(-3 TO 2) AS num_t, b(1, 2) AS DOUBLE
DIM parent(1) AS test
n = 7
REDIM d(3 TO n) AS INTEGER
PRINT LEN(a()); LEN(f()); LEN(na()); LEN(b()); LEN(d())
PRINT LEN(parent(0).a()); LEN(parent())
param a()
SUB param (x() AS INTEGER)
    PRINT LEN(x())
END SUB
