' TEST: check-fail
$CONSOLE:ONLY
' Forms of VARPTR, VARSEG, SADD and VARPTR$ the old compiler rejects, one a line (m4-memory-records design D2,
' D3; verification\v28_x55-x57, v56_t02, v56_x01-x05, x07, x09), and of a STRING member where the old compiler's
' C++ build fails (m4-left-marked design D3, v60_d7, d8): each is an error here too. Then the forms left marked
' (a variable of a SUB compiles since qbasic-testcases): a whole array that is not of numbers, VARSEG of a _BIT array, VARPTR$ of a member of an element
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM i AS INTEGER, l AS LONG, t AS STRING, fx AS STRING * 4, r AS rec, rs AS ts, s AS STRING
DIM a(10) AS INTEGER, fa(10) AS STRING * 4, ra(10) AS rec, ba(10) AS _BIT, sa(10) AS STRING, rsa(3) AS ts
l = SADD(fx)
l = SADD("a")
l = SADD(i)
l = SADD(rs.n)
l = SADD(fa(1))
l = SADD(r)
s = VARPTR$(a(1))
s = VARPTR$(fa(1))
s = VARPTR$(ra(1))
l = VARPTR(r)
l = VARPTR(ba(1))
l = VARPTR(rsa(2).n)
s = VARPTR$(rs.n)
l = VARPTR(sa())
l = VARSEG(ba(1))
s = VARPTR$(ra(1).y)
SYSTEM
