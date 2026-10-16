' TEST: check-fail
$CONSOLE:ONLY
' The forms of FIELD, LSET and RSET the old compiler rejects (m3-console-files task 6.1; verification\v30_x100-
' x105, x107, x108, x110-x123, x125, x128, x131, x140-x152, x154-x160, x162-x164, x167, x168), one line each: each
' is an error that is not "not supported yet"
TYPE t
    s AS STRING * 4
    v AS STRING
    n AS LONG
    m(2) AS LONG
END TYPE
DIM a AS STRING, b AS STRING, x AS STRING * 4, n AS INTEGER, r AS t, r2 AS t, e(3) AS STRING
DIM l AS LONG, fx(3) AS STRING * 4, na(3) AS LONG
CONST K = "kk"
FIELD #1, 4 AS x
FIELD #1, 4 AS n
FIELD #1, 4 AS "lit"
FIELD #1, 4 AS a + b
FIELD #1, 4 AS SPACE$(4)
FIELD #1, 4 AS K
FIELD #1, 4 AS r.s
FIELD #1, 4 AS fx(1)
FIELD #1, 4 AS r
FIELD
FIELD #1
FIELD #1,
FIELD #1, 4 a
FIELD #1, AS a
FIELD #1, 4 AS
FIELD #1, 4 AS a,
FIELD #1, 2 AS a 2 AS b
FIELD "a", 4 AS a
FIELD #1, "4" AS a
FIELD #, 4 AS a
FIELD #1 4 AS a
CALL FIELD(#1, 4 AS a)
FIELD #1, 4 AS fresh
FIELD #1, 2 AS a; 2 AS b
FIELD #1, 4 AS r.m(1)
LSET n = "a"
LSET l = 5
LSET r = r2
LSET "abc" = "a"
LSET a + b = "a"
LSET
LSET a
LSET a =
LSET = "a"
LSET a = 5
LSET K = "a"
LSET (a) = "x"
LSET SPACE$(3) = "x"
LSET a = b = "x"
CALL LSET(a, "x")
LSET MID$(a, 2, 2) = "x"
RSET n = "a"
RSET r = r2
RSET
RSET a = 5
LSET fresh2 = "x"
LSET r.n = "x"
LSET na(1) = "x"
LSET r = "abcd"
LSET a = r
' A FUNCTION's name with empty parentheses is no array (v30_x173-x175).
FIELD #1, 4 AS fn1$()
LSET fn1$() = "x"
RSET fn1$() = "x"
FUNCTION fn1$
    fn1$ = "a"
END FUNCTION
