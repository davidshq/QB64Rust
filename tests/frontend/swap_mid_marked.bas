' TEST: check-fail
$CONSOLE:ONLY
' Forms of SWAP, the MID$ statement and RANDOMIZE the old compiler accepts that are "not supported yet", each marked
' at the form (m2-builtin-statements tasks 5.3, 5.4; verification\v22_x101, x103): an
' operand in parentheses, two whole arrays, a member of an element as the target of MID$ (as an operand of SWAP
' it compiles since m2-defxxx-common-arrays task 9.1: member_arrays_typed.bas); RANDOMIZE through CALL; SWAP
' through CALL (verification\v26_x128)
TYPE pt
    x AS LONG
    s AS STRING * 3
END TYPE
DIM a AS LONG, b AS LONG, m(2) AS LONG, n(2) AS LONG, va(2) AS pt
SWAP (a), b
SWAP m(), n()
MID$(va(1).s, 1) = "x"
CALL RANDOMIZE(5)
CALL SWAP(a, b)
SYSTEM
