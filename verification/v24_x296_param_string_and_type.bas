$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): string, fixed-length string and user-type array parameters; LBOUND and UBOUND; a FUNCTION.
TYPE pt
    x AS LONG
    y AS INTEGER
END TYPE
DIM s(1 TO 2) AS STRING, f(2) AS STRING * 3, p(2) AS pt, d(2 TO 5, -1 TO 1) AS DOUBLE
tell s(), f(), p()
PRINT s(1); s(2); "["; f(1); "]"; p(2).x; p(2).y
PRINT total#(d()); d(5, 1)
SYSTEM
SUB tell (a() AS STRING, b() AS STRING * 3, c() AS pt)
    a(1) = "one": a(2) = a(1) + "+"
    b(1) = "abcdef"
    c(2).x = 70000: c(2).y = LEN(b(1))
END SUB
FUNCTION total# (m() AS DOUBLE)
    DIM i AS LONG, j AS LONG, t AS DOUBLE
    FOR i = LBOUND(m, 1) TO UBOUND(m, 1)
        FOR j = LBOUND(m, 2) TO UBOUND(m, 2)
            m(i, j) = i * j
            t = t + m(i, j)
        NEXT
    NEXT
    total# = t
END FUNCTION
