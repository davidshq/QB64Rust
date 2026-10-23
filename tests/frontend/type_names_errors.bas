' TEST: check-fail
$CONSOLE:ONLY
' AS clauses that name no type (m4-upstream-rejections, tasks 2.1 and 2.2), as measured (upstream
' types/*_bad_typename_*, arrays/40_err_undefined_type, verification v24_x232, v51_x20 to x27, x60 to x89):
' "Unknown type" for a name that is no type word and no TYPE in DIM, DIM SHARED, REDIM, STATIC, COMMON and AS
' lists, and for `* n` after a word other than STRING and _BIT; "Expected STRING * number/constant" for a word
' glued to STRING, with or without `* n` and _UNSIGNED; "Type cannot be _UNSIGNED" before a name that is no type
' or a `* n`; "Cannot convert type (...) to symbol" in SHARED; "Illegal SUB/FUNCTION parameter" for a parameter,
' an array parameter and a library's. A member is "Undefined type", also of a
' TYPE defined in a later block, and in an AS type list "Expected element-name AS type, ...". A TYPE defined
' below a DIM is the type.
TYPE pt
    x AS LONG
END TYPE
DIM b AS _Bytef
DIM s AS Stringfoobar * 2
DIM s2 AS STRING2 * 2
DIM s3 AS Stringfoobar
REDIM r(3) AS _Bytef
COMMON SHARED c AS _Bytef
DIM AS _Bytef d, e
DIM f AS BYTE
DIM g AS INTEGER * 2
DIM h AS _BYTE * 2
DIM i AS LONG * 4
DIM j AS pt * 2
DIM k AS nosuch * 2
DIM l AS _UNSIGNED nosuch
DIM m AS _UNSIGNED _Bytef
DIM n AS _MEMX
DIM o AS _UNSIGNED Stringfoobar * 2
DIM q AS _UNSIGNED INTEGER * 2
DIM SHARED sh AS _Bytef
DIM ok AS later
ok.y = 1
TYPE later
    y AS LONG
END TYPE
TYPE m1
    a AS nosuch
END TYPE
TYPE m2
    variable AS SINGLE
    array (10) AS notdefined
END TYPE
TYPE m3
    a AS _UNSIGNED nosuch
END TYPE
TYPE m4
    a AS BYTE
END TYPE
TYPE m5
    a AS _Bytef
END TYPE
TYPE m6
    a AS Stringfoobar * 2
END TYPE
TYPE m7
    a AS INTEGER * 2
END TYPE
TYPE m8
    AS nosuch a, b
END TYPE
TYPE m9
    a AS m10
END TYPE
TYPE m10
    b AS LONG
END TYPE
SUB s1
    STATIC t AS _Bytef
    STATIC u AS Stringfoobar * 2
    SHARED w AS _Bytef
    REDIM r2(3) AS nosuch
    STATIC AS nosuch x1, x2
END SUB
SUB s2 (p AS _Bytef)
END SUB
FUNCTION f2 (q AS nosuch)
END FUNCTION
SUB s3 (p AS Stringfoobar)
END SUB
SUB s4 (p() AS _Bytef)
END SUB
DECLARE LIBRARY
    FUNCTION labs& (BYVAL n AS _Bytef)
END DECLARE
