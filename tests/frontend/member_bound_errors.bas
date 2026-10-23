' TEST: check-fail
$CONSOLE:ONLY
' A member array's bound that names no CONST above its block (m4-upstream-rejections, task 3.1): the old
' compiler's "TYPE member array bounds must be numeric constant" (measured, upstream
' arrays/39_err_member_must_be_const, verification v24_x195, v51_x01 to x08): a variable, an unknown name, a
' CONST below the block or in a SUB below it, LEN, an expression with a variable, a variable with a suffix. A
' bound that names a CONST above the block, also in an included file, or may be one (_PI, a function of the
' constant evaluator) keeps its mark alone (v24_x196, v51_x09, x10).
'$INCLUDE:'inc/member_bound.bi'
DIM AS _BYTE a
a = 1
n = 3
CONST top = 3
TYPE t1
    variable AS SINGLE
    array (a) AS LONG
END TYPE
TYPE t2
    v(n) AS LONG
END TYPE
TYPE t3
    v(zz) AS LONG
END TYPE
TYPE t4
    v(below) AS LONG
END TYPE
CONST below = 2
TYPE t5
    v(LEN("abc")) AS LONG
END TYPE
TYPE t6
    v(1 TO n + 1) AS LONG
END TYPE
TYPE t7
    v(n%) AS LONG
END TYPE
TYPE t8
    v(top) AS LONG
END TYPE
TYPE t9
    v(top%) AS LONG
END TYPE
TYPE t10
    v(_PI) AS LONG
END TYPE
TYPE t11
    v(incl) AS LONG
END TYPE
TYPE t12
    v(2 * top) AS LONG
END TYPE
TYPE t13
    v(subtop) AS LONG
END TYPE
SUB s
    CONST subtop = 3
END SUB
