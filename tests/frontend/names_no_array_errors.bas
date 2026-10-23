' TEST: check-fail
$CONSOLE:ONLY
' Names that are no array (m4-names-implicit-arrays task 2.2): the old compiler's errors as real errors (measured,
' verification\v24_x106, x112, x119, v22_x79, v26_x120, v28_x53, v33_x29, v44_x46, v49_x16, v55_x09 to x20,
' x54, x55, x76 to x82). The lines between the errors are accepted.
DIM a AS LONG, n AS LONG, s AS STRING
DATA 1, 2
' "Invalid array name": one leading underscore and a spelling that calls no built-in function.
_foo(1) = 2
n = _foo(1)
n = _TRUE(1)
PRINT _CRC32$("x")
n = _CAST&(LONG, 1)
PRINT _OPENCLIENT&("x")
s = _IIF$(-1, "yes", "no")
PRINT UBOUND(_foo)
' Accepted: a spelling that calls the function, two leading underscores.
PRINT _CONNECTIONADDRESS$(0); _CONNECTIONADDRESS(0)
__foo(1) = 2
' A built-in function's name as a place or a target.
rnd(1) = 2
TAB(5) = 1
sin(1) = 2
CHR$(1) = "a"
_RGB(1) = 2
READ ABS(a)
READ TAB(1)
INPUT ABS(a)
LINE INPUT CHR$(1)
SWAP ABS(a), n
PRINT UBOUND(sin)
PRINT LEN(sin())
' A SUB's or FUNCTION's name as a place, a target or a value with an index.
f(3) = 5
READ f(1)
INPUT f(1)
sb(3) = 5
n = sb(1)
SYSTEM

FUNCTION f (k)
    f = k * 2
END FUNCTION

SUB sb
END SUB

SUB below
    f(3) = 5
END SUB
