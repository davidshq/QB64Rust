' TEST: check-fail
$CONSOLE:ONLY
' OPTION _EXPLICIT errors (m2-control-flow-slice D7), each rejected by the old compiler too (verification\v17_f_*):
' an implicit variable before the OPTION line, an earlier SUB's implicit variable, an assignment and a read of an
' undeclared variable, `x%` after `DIM x AS LONG`, a SUB's undeclared local, `SHARED` naming nothing in main, a
' `SHARED` before the main module's DIM, a `SHARED` without AS of a main LONG, `OPTION EXPLICIT` without `_`.
z = 1
SUB early
    y = 1
END SUB
OPTION _EXPLICIT
a = 1
PRINT b
DIM x AS LONG
x& = 1
x% = 2
s
SYSTEM

SUB s
    w = 1
    SHARED never AS LONG
    SHARED later AS LONG
    SHARED x
END SUB

DIM later AS LONG
OPTION EXPLICIT
