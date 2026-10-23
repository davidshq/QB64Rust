' TEST: cpp
$CONSOLE:ONLY
' _CAST in the C++ (m4-builtins-param-types task 3.2, design D2; verification/v49_a_cast): a C++ cast to the type
' named, of a variable, a literal, a member and an element, never folded; the result in an expression of its type.
' A BYVAL argument of a library keeps the cast, though a conversion C makes the same way is left out around it.
TYPE pt
    x AS LONG
END TYPE
DECLARE LIBRARY
    FUNCTION labs& (BYVAL v AS LONG)
    FUNCTION fabs# (BYVAL v AS DOUBLE)
END DECLARE
DIM sg AS SINGLE, d AS DOUBLE, k AS LONG, q AS _INTEGER64, u AS _UNSIGNED LONG, b AS _BYTE, p AS pt, a(2) AS SINGLE
PRINT _CAST(_BYTE, sg)
k = _CAST(_UNSIGNED _BYTE, k)
PRINT _CAST(INTEGER, d * 2); _CAST(_UNSIGNED INTEGER, 70000.5)
PRINT _CAST(LONG, q); _CAST(_UNSIGNED LONG, b); _CAST(_INTEGER64, sg); _CAST(_UNSIGNED _INTEGER64, b)
PRINT _CAST(_OFFSET, k); _CAST(_UNSIGNED _OFFSET, d)
PRINT _CAST(SINGLE, q); _CAST(DOUBLE, k); _CAST(_FLOAT, u)
PRINT _CAST(INTEGER, p.x); _CAST(_BYTE, a(1) * 2)
PRINT _CAST(_BYTE, 100) + _CAST(_BYTE, 100); _CAST(LONG, 2.5)
PRINT labs(_CAST(_BYTE, k)); fabs(_CAST(LONG, sg))
SYSTEM
