' TEST: check-fail
$CONSOLE:ONLY
' COMMON (m2-defxxx-common-arrays task 7.2): the old compiler's errors as real errors (measured,
' verification\v24_x162, x164, x165, x170, x171, x174, x175)
' "Name already in use": COMMON of a scalar twice, and DIM or DIM SHARED of a COMMON scalar.
COMMON x
COMMON x
COMMON y AS LONG
DIM y AS LONG
COMMON SHARED z AS LONG
DIM SHARED z AS LONG
' "Invalid variable name": a block name, with and without SHARED and without a variable.
COMMON /blk/ b1 AS LONG
COMMON SHARED /blk/ b2 AS LONG
' A name with a suffix and AS; the same number of dimensions as the DIM gave.
COMMON q% AS LONG
COMMON a() AS LONG
DIM a(3) AS LONG
a(1, 2) = 5
' "COMMON cannot be used within a SUB/FUNCTION".
SUB s
    COMMON inside
END SUB
