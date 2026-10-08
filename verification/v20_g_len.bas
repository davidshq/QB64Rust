$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): LEN of a string expression and of places of each kind.
TYPE inner
    a AS INTEGER
    b AS DOUBLE
END TYPE
TYPE outer
    n AS LONG
    s AS inner
    f AS _FLOAT
END TYPE
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT, b AS _BYTE
DIM s AS STRING, o AS outer, n AS inner
DIM ai(3) AS INTEGER, ad(3) AS DOUBLE, ao(3) AS outer, sa(3) AS STRING
s = "hello"
sa(1) = "abc"
PRINT "LEN string var, expr, literal:"; LEN(s); LEN(s + "!"); LEN("abc"); LEN("")
PRINT "LEN numeric vars:"; LEN(i); LEN(l); LEN(q); LEN(f); LEN(d); LEN(x); LEN(b)
PRINT "LEN user vars:"; LEN(o); LEN(n)
PRINT "LEN members:"; LEN(o.n); LEN(o.s); LEN(o.s.b); LEN(o.f)
PRINT "LEN elements:"; LEN(ai(1)); LEN(ad(2)); LEN(ao(1)); LEN(ao(1).s); LEN(ao(1).s.a); LEN(sa(1)); LEN(sa(2))
PRINT "LEN implicit:"; LEN(z); LEN(z$); LEN(z#); LEN(z&&)
PRINT "LEN typed tree: "; LEN(s) / 3; LEN(i) / 3
SYSTEM
