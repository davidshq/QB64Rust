' TEST: parse-ok
$CONSOLE:ONLY
' Declarations (m2-parser-breadth task 7.2, design D5): every form found in the inputs parses; sema marks what it
' does not compile yet (REDIM, COMMON, ERASE, DEFxxx, _DEFINE, DIM AS type lists, fixed-length strings, member
' arrays, array parameters, STATIC after a header, type names as arguments, blanks around a dot).
DEFINT A-Z
DEFLNG i-k, n
DEFSNG s
DEFDBL d - e
DEFSTR t
_DEFINE m-m AS _UNSIGNED LONG
DIM AS LONG a1, a2(1 TO 3)
DIM SHARED AS STRING * 8 f1, f2
DIM s1 AS STRING * 10, s2 AS STRING * size
DIM d1() AS DOUBLE
REDIM r1(10)
REDIM _PRESERVE r1(1 TO 20) AS LONG
REDIM SHARED _PRESERVE r2(5, 5)
REDIM _RETAIN r3(0 TO 2, 6 TO 9) AS _UNSIGNED _BIT * 5
REDIM item(0).values(0 TO 3)
REDIM _PRESERVE w(1).g(0).n(1).d(0 TO 3)
REDIM AS INTEGER r4(3), r5(4)
COMMON SHARED c1, c2() AS INTEGER
COMMON /blk/ c3
ERASE r1, r2(), x(0).s, a . s
STATIC AS LONG st1
v = VAL("18", _UNSIGNED _INTEGER64)
v = _MEMGET(m, m.OFFSET, _UNSIGNED INTEGER)
PRINT _CAST(_BYTE, v)
a . s = 5
PRINT a . s
SYSTEM

SUB arr (x(), y( ,) AS LONG, z AS INTEGER) STATIC
    STATIC AS LONG st2, st3
    SHARED AS DOUBLE sh1
END SUB

FUNCTION f& (q() AS STRING)
    f& = 1
END FUNCTION
