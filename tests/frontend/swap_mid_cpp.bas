' TEST: cpp
$CONSOLE:ONLY
' SWAP, the MID$ statement, RANDOMIZE, RND and TIMER as C++ (m2-builtin-statements tasks 5.3, 5.4, design D5; the
' old compiler's lines are in study\00 section 5): swap_8, swap_16, swap_32, swap_64 by the width (signedness and
' float or not do not count), swap_longdouble for _FLOAT, swap_string for strings of any kind, swap_block for a
' TYPE with the second place first and its size; an element's index checked inside the argument; sub_mid(target,
' start,length,value,passed) with 0,…,0 for a length left out; sub_randomize(seed,1), (seed,3) with USING, (NULL,0) bare;
' func_rnd(NULL,0) and func_rnd(x,1); func_timer(NULL,0) and func_timer(x,1)
TYPE rec
    n AS LONG
    s AS STRING * 3
END TYPE
DIM b1 AS _BYTE, b2 AS _BYTE, i1 AS INTEGER, i2 AS INTEGER, l1 AS LONG, l2 AS LONG, ul AS _UNSIGNED LONG
DIM q1 AS _INTEGER64, q2 AS _INTEGER64, s1 AS SINGLE, s2 AS SINGLE, d1 AS DOUBLE, d2 AS DOUBLE
DIM f1 AS _FLOAT, f2 AS _FLOAT, o1 AS _OFFSET, o2 AS _OFFSET, a AS STRING, c AS STRING
DIM x1 AS STRING * 3, x2 AS STRING * 5, r1 AS rec, r2 AS rec, arr(3) AS LONG, sarr(3) AS STRING, rarr(2) AS rec
DIM k AS LONG
SWAP b1, b2
SWAP i1, i2
SWAP l1, ul
SWAP q1, q2
SWAP s1, s2
SWAP d1, d2
SWAP f1, f2
SWAP o1, o2
SWAP a, c
SWAP x1, x2
SWAP a, x1
SWAP r1, r2
SWAP arr(k), arr(k + 1)
SWAP l1, arr(1)
SWAP sarr(0), sarr(k)
SWAP r1.n, l1
SWAP r1.s, x1
SWAP rarr(0), rarr(1)
MID$(a, 3) = "XY"
MID$(a, k, 2) = c
MID$(a, d1, d1) = c + "x"
MID$(x1, 2) = "Z"
MID$(sarr(k), 2, 2) = "XY"
MID$(r1.s, 2, 2) = "XY"
RANDOMIZE 5
RANDOMIZE USING 5
RANDOMIZE d1
RANDOMIZE USING l1
RANDOMIZE TIMER
RANDOMIZE
d1 = RND
d1 = RND(1)
d1 = RND(l1)
d1 = TIMER
d1 = TIMER(.001)
d1 = TIMER(l1)
PRINT RND; TIMER; TIMER(.001)
s1 = RND * 2
SYSTEM
