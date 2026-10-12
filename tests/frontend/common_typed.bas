' TEST: typed
$CONSOLE:ONLY
' COMMON (m2-defxxx-common-arrays task 7.2, design D6; measured, verification\v24_g_common, v24_a_common, v24_x161
' to x175, x181): a scalar is declared as by DIM, with SHARED as by DIM SHARED, of any type, also with the type
' before the names, and below executable statements. COMMON a() lists a dynamic array that is not allocated
' (what stands between its parentheses is not read): a DIM or REDIM of that name and type below is an Allocate,
' also a DIM SHARED with constant bounds; with another type it is another array, which is not shared. A scalar
' and an array of one name are two variables. Nothing is written for CHAIN
TYPE pt
    x AS LONG
END TYPE
n = 3
COMMON SHARED total AS LONG, word$, w AS pt, fx AS STRING * 4
COMMON v AS LONG
COMMON AS LONG la, lb()
COMMON SHARED cs() AS LONG, ign(3) AS LONG, other()
COMMON sc, never() AS DOUBLE
DIM cs(n) AS LONG
DIM SHARED ign(2, 5) AS LONG
DIM other(3) AS LONG
DIM lb(1 TO 2) AS LONG
DIM sc(3)
REDIM cs(9) AS LONG
total = 5: v = total: w.x = 1: fx = "abcdef": sc = 1: sc(1) = 2
other(1) = la + lb(1)
PRINT never(1)
show
SYSTEM
SUB show
    PRINT total; word$; w.x; fx; cs(2); ign(1, 2)
    PRINT v; other(2)
END SUB
