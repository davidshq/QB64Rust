' TEST: typed
$CONSOLE:ONLY
' The new numeric types everywhere else (m2-numeric-types task group 8): an array and a member of them, a variable,
' an element and a member passed by reference to a parameter of the other signedness (`Ref`, measured
' verification\v21_b_passing, v21_g_passing_places), an `_INTEGER64` to an `_UNSIGNED _OFFSET` parameter, an
' unsigned FUNCTION and an f$n FUNCTION, FOR variables (the hidden type by width), SELECT CASE selectors (the copy's
' type), constants used with another suffix (v21_g_const_suffix) and the special-cased built-ins (v21_d_builtins).
TYPE nt
    ub AS _UNSIGNED _BYTE
    ul AS _UNSIGNED LONG
    o AS _OFFSET
END TYPE
DIM v AS nt, a(2) AS _UNSIGNED INTEGER, sl AS LONG, q AS _INTEGER64
v.ub = -1: a(1) = -1
showul sl
showul v.ul
showsi a(1)
showsi (a(1))
showuo q
PRINT big~&(255); fs$5("ab")
FOR b~%% = 250 TO 255 STEP 3: NEXT
FOR u~% = 2 TO 0 STEP -1: NEXT
FOR l~& = 1 TO 2: NEXT
SELECT CASE v.ub
    CASE -1: PRINT "never"
    CASE 254.5 TO 255: PRINT "255"
END SELECT
SELECT CASE q + 1~&&
    CASE 1: PRINT "one"
END SELECT
CONST neg = -1, cu~& = 4294967295
PRINT neg~&; neg~& + 0; cu%; cu% + 0
PRINT HEX$(v.ub); ABS(v.ub); SGN(v.ub); INT(v.ul); CINT(a(2)); CLNG(v.ul); _ROUND(v.o); SQR(v.ub); EXP(v.ul)
PRINT VAL("-1", _UNSIGNED LONG); VAL("-1", _OFFSET)
SYSTEM

SUB showul (x AS _UNSIGNED LONG)
    x = 5
END SUB

SUB showsi (x AS INTEGER)
    x = -3
END SUB

SUB showuo (x AS _UNSIGNED _OFFSET)
    x = 7
END SUB

FUNCTION big~& (x AS _UNSIGNED _BYTE)
    big~& = x * 16843009
END FUNCTION

FUNCTION fs$5 (x AS STRING)
    fs$5 = x
END FUNCTION
