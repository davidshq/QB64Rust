' TEST: check-fail
$CONSOLE:ONLY
' Declarations and literals of the new numeric types that the old compiler rejects (m2-numeric-types tasks 4.1 to
' 4.3; verification\v21_x01-x19, x20-x30, x31-x34, x35-x48): each line one error, none "not supported yet". A
' fixed-length string parameter's length is read in 32 bits: 4294967296 is 0. A `_BIT` FUNCTION is declared, but
' every call of it is "Name already in use".
DIM s AS _UNSIGNED SINGLE
DIM u AS _UNSIGNED
DIM b0 AS _BIT * 0
DIM b65 AS _BIT * 65
CONST w = 3
DIM bw AS _BIT * w
DIM y%% AS _BYTE
k`65 = 1
k`0 = 1
DIM t AS _BIT * 4
PRINT LEN(t)
FOR i` = 1 TO 2: NEXT
PRINT VAL("1", _BIT)
PRINT 5%&
PRINT &H1FF%%
PRINT &H1FFFFFFFF&
PRINT 18446744073709551616~&&
TYPE withbit
    m AS _BIT
END TYPE
TYPE pair
    a AS LONG
END TYPE
DIM up AS _UNSIGNED pair
PRINT fb`(1)
PRINT fz`
SYSTEM

FUNCTION fb` (x)
END FUNCTION

FUNCTION fz`
END FUNCTION

SUB bitparam (p AS _BIT * 3)
END SUB

SUB ustring (t AS _UNSIGNED STRING)
END SUB

SUB ufixed (t AS _UNSIGNED STRING * 3)
END SUB

SUB zero (t$0)
END SUB

SUB wraps (t AS STRING * 4294967296)
END SUB
