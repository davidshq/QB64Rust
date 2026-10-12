' TEST: check-ok
$CONSOLE:ONLY
' OPTION _EXPLICIT with every declaration kind (m2-control-flow-slice D7, verification\v17_f_explicit_kinds,
' v17_f_explicit_const_in_sub): DIM, CONST, DIM SHARED, SHARED of a declared main variable, STATIC, a parameter,
' the FUNCTION's own name, a SUB's own CONST and a main CONST in a SUB. Also: the OPTION after another statement,
' twice, with a statement after a colon, together with _EXPLICITARRAY; `x&` after `DIM x AS LONG` is `x`.
PRINT 1
OPTION _EXPLICIT: DIM x AS LONG
OPTION _EXPLICIT
OPTION _EXPLICITARRAY
CONST k = 2
DIM SHARED g AS LONG
DIM d%
x = k
x& = x& + 1
g = 5
d% = 3
s 7
PRINT x; g; d%; f
SYSTEM

SUB s (p)
    SHARED x AS LONG
    STATIC st
    CONST own = 4
    st = st + 1
    p = p + 1
    PRINT p; st; x; g; k; own
END SUB

FUNCTION f
    f = 3
END FUNCTION
