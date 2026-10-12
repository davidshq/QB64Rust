' TEST: ir
$CONSOLE:ONLY
' Array storage in the IR (m2-defxxx-common-arrays task 4.1, spec compiler/pipeline "Array storage in the IR"): an
' array is static, with its bounds, or dynamic, with its number of dimensions. A static DIM is no operation; a DIM
' with a bound that is not constant, a REDIM and a REDIM _PRESERVE are an Allocate with its mode and its bounds as
' 64-bit values, marked as raising; ERASE is a Clear of a static array and a Release of a dynamic one. No
' descriptor, allocation function or size in bytes appears.
n = 4
DIM d(n) AS LONG
DIM s(5) AS LONG
REDIM _PRESERVE d(9) AS LONG
REDIM t(1 TO n, 2.5) AS STRING
DIM s(n) AS LONG
d(1) = UBOUND(t, 2)
ERASE s, d
resize
SYSTEM
'$DYNAMIC
DIM SHARED g(3) AS DOUBLE
SUB resize
    REDIM g(LEN("ab")) AS DOUBLE
    ERASE g
END SUB
