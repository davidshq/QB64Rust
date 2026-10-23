' TEST: check-fail
$CONSOLE:ONLY
' LBOUND and UBOUND of an element (m4-upstream-rejections, task 4.1), as measured (upstream
' arrays/42_err_solved_bug_Incorrect_nr_of_args, 43_...; verification v51_x28 to x39): an element of a member
' array, of a single value's or an array element's, with or without a dimension, is "Incorrect number of
' arguments"; an element of an array of a TYPE "Expected ."; of another array, a FUNCTION's call, a literal
' and an operation "Syntax error". A parenthesised array name keeps the mark (the old compiler takes it).
TYPE g1
    a(1 TO 5) AS LONG
END TYPE
TYPE e
    a(1 TO 50, 2 TO 20) AS _BYTE
    g AS g1
END TYPE
DIM f(15) AS e
DIM v AS e
DIM n(10) AS LONG
DIM t(10) AS STRING
PRINT UBOUND(f(10).a(5), 1)
PRINT LBOUND(f(10).a(5), 1)
PRINT LBOUND(f(10).a(5))
PRINT UBOUND(f(10).a(5, 3), 2)
PRINT UBOUND(v.a(5))
PRINT LBOUND(f(1).g.a(2))
PRINT LBOUND(f(1))
PRINT LBOUND(n(5))
PRINT LBOUND(5)
PRINT LBOUND(t(5))
PRINT LBOUND(fn(1))
PRINT LBOUND(n + 1)
PRINT LBOUND(f(10).a, 2); UBOUND(f(10).a, 2)
PRINT LBOUND((n))
FUNCTION fn (x)
    fn = x
END FUNCTION
