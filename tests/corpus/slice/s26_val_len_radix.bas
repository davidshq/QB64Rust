$CONSOLE:ONLY
' Slice program (m2-core-builtins, D9): VAL with and without a type, LEN of string expressions and of places of
' each kind, HEX$/OCT$/_BIN$ with the width of a negative value from the argument's type (verification\v20_d_val,
' v20_e_radix, v20_g_len). Errors are trapped: the handler prints ERR and resumes next. No PRINT comma.
ON ERROR GOTO h
TYPE inner
    a AS INTEGER
    b AS DOUBLE
END TYPE
TYPE outer
    n AS LONG
    s AS inner
    f AS _FLOAT
END TYPE
' VAL
PRINT VAL("12"); VAL("-3.5"); VAL(""); VAL("&H"); VAL("&HFF"); VAL("&HFFFF"); VAL("&HFFFFFFFF")
PRINT VAL("&O17"); VAL("&O"); VAL("&B101"); VAL("&B"); VAL(" 1 2"); VAL(CHR$(9) + "3"); VAL("1e3"); VAL("1d3")
PRINT VAL("abc"); VAL("12abc"); VAL("+5"); VAL(".5"); VAL("1,5"); VAL("0.1"); VAL("0.333333333333333333333333")
PRINT VAL("1") / 3; VAL("1", SINGLE) / 3; VAL("1", DOUBLE) / 3; VAL("1", _FLOAT) / 3
PRINT VAL("40000", INTEGER); VAL("2.7", INTEGER); VAL("-2.5", INTEGER); VAL("3000000000", LONG); VAL("1e3", LONG)
PRINT VAL("9007199254740993", _INTEGER64); VAL("1.5", _INTEGER64); VAL("x", LONG)
PRINT VAL("0.1", SINGLE); VAL("0.1", DOUBLE); VAL("0.1", _FLOAT)
PRINT VAL("16777217", SINGLE) = 16777216&&; VAL("16777217", DOUBLE) = 16777216&&
PRINT HEX$(VAL("-1", INTEGER)) + " " + HEX$(VAL("-2", LONG))
' LEN
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
DIM s AS STRING, o AS outer, n AS inner
DIM ai(3) AS INTEGER, ad(3) AS DOUBLE, ao(3) AS outer, sa(3) AS STRING
s = "hello"
sa(1) = "abc"
PRINT LEN(s); LEN(s + "!"); LEN("abc"); LEN(""); LEN(STR$(12))
PRINT LEN(i); LEN(l); LEN(q); LEN(f); LEN(d); LEN(x)
PRINT LEN(o); LEN(n); LEN(o.n); LEN(o.s); LEN(o.s.b); LEN(o.f)
PRINT LEN(ai(1)); LEN(ad(2)); LEN(ao(1)); LEN(ao(1).s); LEN(ao(1).s.a); LEN(sa(1)); LEN(sa(2))
PRINT LEN(z); LEN(z$); LEN(z#); LEN(z&&); LEN(s) / 3; LEN(i) / 3
' HEX$, OCT$, _BIN$: the width of a negative value.
i = -2: l = -2: q = -2: ai(1) = -2: n.a = -2
PRINT HEX$(i) + " " + HEX$(l) + " " + HEX$(q) + " " + HEX$(ai(1)) + " " + HEX$(n.a)
PRINT OCT$(i) + " " + OCT$(l) + " " + OCT$(q) + " " + OCT$(ai(1)) + " " + OCT$(n.a)
PRINT _BIN$(i) + " " + _BIN$(l) + " " + _BIN$(q)
PRINT HEX$(i * 1) + " " + HEX$(l * 1) + " " + HEX$(q * 1) + " " + HEX$(i AND i) + " " + HEX$(-i)
PRINT OCT$(i * 1) + " " + OCT$(l * 1) + " " + _BIN$(q * 1)
i = -1: l = -1: q = -1
PRINT "[" + HEX$(i * 1) + "][" + HEX$(q * 1) + "][" + HEX$(q) + "][" + OCT$(q * 1) + "][" + _BIN$(l * 1) + "]"
PRINT HEX$(-1) + " " + HEX$(-1%) + " " + HEX$(-1&) + " [" + HEX$(-1&&) + "] " + HEX$(255) + " " + HEX$(-70000)
PRINT HEX$(CINT(-2)) + " " + HEX$(CLNG(-2)) + " " + HEX$(SGN(-2)) + " " + HEX$(ASC("a") - 200) + " " + HEX$(LEN(s) - 9)
PRINT HEX$(0) + " " + OCT$(0) + " " + _BIN$(0) + " " + HEX$(48879) + " " + OCT$(8) + " " + _BIN$(5)
f = -2.5: d = 2.5: x = 1E+30
PRINT HEX$(f) + " " + HEX$(d) + " " + HEX$(-1.5) + " " + HEX$(3.5#) + " " + OCT$(f) + " " + _BIN$(d)
PRINT "["; HEX$(x); "]"
PRINT "["; OCT$(x); "]"
PRINT "["; _BIN$(x); "]"
PRINT HEX$(2## ^ 62) + " " + HEX$(-(2## ^ 62))
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
