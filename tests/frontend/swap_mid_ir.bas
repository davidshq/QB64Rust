' TEST: ir
$CONSOLE:ONLY
' SWAP, the MID$ statement and RANDOMIZE in the IR (m2-builtin-statements tasks 5.3, 5.4, design D2): SWAP names
' its two places, with no temporary and no address; the MID$ statement its string place, the start, the length or
' an absent slot, and the value; RANDOMIZE the word USING or an absent slot, and the seed converted to DOUBLE. Each
' is one built-in operation that may raise
TYPE rec
    n AS LONG
    s AS STRING * 3
END TYPE
DIM a AS LONG, b AS LONG, u AS _UNSIGNED LONG, s AS STRING, t AS STRING, fx AS STRING * 3
DIM x(3) AS INTEGER, r1 AS rec, r2 AS rec, i AS LONG
SWAP a, b
SWAP a, u
SWAP s, t
SWAP s, fx
SWAP x(i), x(i + 1)
SWAP r1, r2
SWAP r1.n, a
MID$(s, 3) = "XY"
MID$(s, i, 2) = t
MID$(r1.s, 2.5, 1) = t + "x"
RANDOMIZE 5
RANDOMIZE USING a
RANDOMIZE TIMER
p
SYSTEM

SUB p
    DIM c AS DOUBLE, d AS DOUBLE, q AS STRING
    SWAP c, d
    MID$(q, 1) = "z"
    RANDOMIZE c
END SUB
