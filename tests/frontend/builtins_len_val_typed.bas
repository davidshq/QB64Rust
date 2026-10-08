' TEST: typed
$CONSOLE:ONLY
' LEN, ASC, VAL, HEX$, OCT$, _BIN$ by their rules (m2-core-builtins task 4.2): LEN of a string expression (a call)
' and of places of each kind (their size, a constant; the index of an element not evaluated); ASC with one and two
' arguments; VAL _FLOAT, with SINGLE and DOUBLE as named and with any integer type _INTEGER64; the radix functions
' take their argument as it is
TYPE inner
    a AS INTEGER
    b AS DOUBLE
END TYPE
TYPE outer
    n AS LONG
    s AS inner
    f AS _FLOAT
END TYPE
DIM s AS STRING, i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
DIM o AS outer, ai(3) AS INTEGER, ao(3) AS outer, sa(3) AS STRING
PRINT LEN(s); LEN(s + "!"); LEN(sa(1)); LEN("")
PRINT LEN(i); LEN(l); LEN(q); LEN(f); LEN(d); LEN(x); LEN(z)
PRINT LEN(o); LEN(o.n); LEN(o.s); LEN(o.s.b); LEN(ai(i)); LEN(ao(1)); LEN(ao(1).s.a)
' A LEN in the index of a whole TYPE element (verification\v20_j_len_nested)
PRINT LEN(ao(LEN(i))); LEN(ao(LEN(o)).s)
PRINT ASC(s); ASC(s, 2); ASC(s, 2.5); ASC(s, q)
PRINT VAL(s); VAL(s, SINGLE); VAL(s, DOUBLE); VAL(s, _FLOAT); VAL(s, INTEGER); VAL(s, LONG); VAL(s, _INTEGER64)
PRINT HEX$(i); HEX$(q); HEX$(q * 1); HEX$(f); OCT$(l); _BIN$(i + 1)
