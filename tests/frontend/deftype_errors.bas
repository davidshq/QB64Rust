' TEST: check-fail
$CONSOLE:ONLY
' Default types (m2-defxxx-common-arrays task 2.1): the old compiler's errors as errors (verification\v24_x05 to
' x26, x34 to x37): types that cannot be a default, malformed statements, and the ordinary errors of the type a
' default gives (a STRING FOR variable, a number stored into a STRING, a _BIT parameter, a FUNCTION called with
' the suffix of another type). _MEM is a user type, so it cannot be a default either ("Cannot convert type (_MEM)
' to symbol", m3-mem); an unknown type is "not supported yet", as in DIM, and stands last because a marked
' declaration drops the real errors after it.
TYPE rec
    n AS LONG
END TYPE
_DEFINE a AS rec
_DEFINE a AS _UNSIGNED STRING
_DEFINE a AS _UNSIGNED SINGLE
_DEFINE a AS STRING * 0
_DEFINE a AS _UNSIGNED
_DEFINE a AS _BIT * 65
_DEFINE a
_DEFINE AS LONG
_DEFINE a AS INTEGER, b AS LONG
DEFINT
DEFINT 1
DEFINT ab
DEFINT a-
DEFINT a b
DEFINT a,
DEFINT a AS LONG
a1 = 2.5
DEFSTR S
FOR s = 1 TO 3: NEXT
s2 = 5
DEFLNG F
PRINT f!(3)
_DEFINE p AS _BIT
SYSTEM
FUNCTION f (x)
    f = x * 100000
END FUNCTION
SUB show (p1)
END SUB
_DEFINE q AS _MEM
_DEFINE q AS nosuch
