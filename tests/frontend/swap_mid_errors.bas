' TEST: check-fail
$CONSOLE:ONLY
' SWAP, the MID$ statement, RANDOMIZE, RND and TIMER rejected (verification\v22_x91-x136): SWAP of two types that
' differ (LONG and DOUBLE, INTEGER and LONG, SINGLE and DOUBLE, _OFFSET and _INTEGER64, a string and a number, two
' TYPEs), of an operand that is no variable (a literal, an expression, a CONST), of _BIT variables, with one operand
' or three; the MID$ statement with a target that is no string variable (a literal, a number, an expression, a
' CONST), a string start, a number as the value, one argument or four; RANDOMIZE with a string; RND and TIMER with
' two arguments, a string, empty parentheses, as a variable. Left out and "not supported yet" (swap_mid_marked.bas)
CONST c = 1
CONST cs = "abc"
TYPE r1
    n AS LONG
END TYPE
TYPE r2
    n AS LONG
END TYPE
DIM a AS LONG, b AS LONG, d AS DOUBLE, s AS STRING, t AS STRING, i AS INTEGER, sg AS SINGLE
DIM o AS _OFFSET, q AS _INTEGER64, x AS r1, y AS r2, t1 AS _BIT, t2 AS _BIT, w3 AS _BIT * 3, w5 AS _BIT * 5
SWAP a, d
SWAP i, a
SWAP sg, d
SWAP o, q
SWAP s, a
SWAP x, y
SWAP x, a
SWAP a, 5
SWAP a, b + 1
SWAP a, c
SWAP t1, t2
SWAP w3, w5
SWAP a
SWAP a, b, a
MID$("abc", 1) = "x"
MID$(a, 1) = "x"
MID$(s + t, 1) = "x"
MID$(cs, 1) = "x"
MID$(s, "1") = "x"
MID$(s, 1, "1") = "x"
MID$(s, 1) = 5
MID$(s) = "x"
MID$(s, 1, 1, 1) = "x"
RANDOMIZE "a"
PRINT RND(1, 2)
PRINT RND("a")
PRINT RND()
PRINT TIMER(1, 2)
PRINT TIMER("a")
PRINT TIMER()
RND = 1
TIMER = 1
SYSTEM

FUNCTION f&
    DIM l AS LONG
    SWAP f&, l
END FUNCTION

FUNCTION g$
    MID$(g$, 1) = "x"
END FUNCTION
