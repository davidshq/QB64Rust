$Console:Only
' Slice program (m2-core-builtins, D9): VAL with and without a type, LEN of string expressions and of places of
' each kind, HEX$/OCT$/_BIN$ with the width of a negative value from the argument's type (verification\v20_d_val,
' v20_e_radix, v20_g_len). Errors are trapped: the handler prints ERR and resumes next. No PRINT comma.
On Error GoTo h
Type inner
    a As Integer
    b As Double
End Type
Type outer
    n As Long
    s As inner
    f As _Float
End Type
' VAL
Print Val("12"); Val("-3.5"); Val(""); Val("&H"); Val("&HFF"); Val("&HFFFF"); Val("&HFFFFFFFF")
Print Val("&O17"); Val("&O"); Val("&B101"); Val("&B"); Val(" 1 2"); Val(Chr$(9) + "3"); Val("1e3"); Val("1d3")
Print Val("abc"); Val("12abc"); Val("+5"); Val(".5"); Val("1,5"); Val("0.1"); Val("0.333333333333333333333333")
Print Val("1") / 3; Val("1", Single) / 3; Val("1", Double) / 3; Val("1", _Float) / 3
Print Val("40000", Integer); Val("2.7", Integer); Val("-2.5", Integer); Val("3000000000", Long); Val("1e3", Long)
Print Val("9007199254740993", _Integer64); Val("1.5", _Integer64); Val("x", Long)
Print Val("0.1", Single); Val("0.1", Double); Val("0.1", _Float)
Print Val("16777217", Single) = 16777216&&; Val("16777217", Double) = 16777216&&
Print Hex$(Val("-1", Integer)) + " " + Hex$(Val("-2", Long))
' LEN
Dim i As Integer, l As Long, q As _Integer64, f As Single, d As Double, x As _Float
Dim s As String, o As outer, n As inner
Dim ai(3) As Integer, ad(3) As Double, ao(3) As outer, sa(3) As String
s = "hello"
sa(1) = "abc"
Print Len(s); Len(s + "!"); Len("abc"); Len(""); Len(Str$(12))
Print Len(i); Len(l); Len(q); Len(f); Len(d); Len(x)
Print Len(o); Len(n); Len(o.n); Len(o.s); Len(o.s.b); Len(o.f)
Print Len(ai(1)); Len(ad(2)); Len(ao(1)); Len(ao(1).s); Len(ao(1).s.a); Len(sa(1)); Len(sa(2))
Print Len(z); Len(z$); Len(z#); Len(z&&); Len(s) / 3; Len(i) / 3
' HEX$, OCT$, _BIN$: the width of a negative value.
i = -2: l = -2: q = -2: ai(1) = -2: n.a = -2
Print Hex$(i) + " " + Hex$(l) + " " + Hex$(q) + " " + Hex$(ai(1)) + " " + Hex$(n.a)
Print Oct$(i) + " " + Oct$(l) + " " + Oct$(q) + " " + Oct$(ai(1)) + " " + Oct$(n.a)
Print _Bin$(i) + " " + _Bin$(l) + " " + _Bin$(q)
Print Hex$(i * 1) + " " + Hex$(l * 1) + " " + Hex$(q * 1) + " " + Hex$(i And i) + " " + Hex$(-i)
Print Oct$(i * 1) + " " + Oct$(l * 1) + " " + _Bin$(q * 1)
i = -1: l = -1: q = -1
Print "[" + Hex$(i * 1) + "][" + Hex$(q * 1) + "][" + Hex$(q) + "][" + Oct$(q * 1) + "][" + _Bin$(l * 1) + "]"
Print Hex$(-1) + " " + Hex$(-1%) + " " + Hex$(-1&) + " [" + Hex$(-1&&) + "] " + Hex$(255) + " " + Hex$(-70000)
Print Hex$(CInt(-2)) + " " + Hex$(CLng(-2)) + " " + Hex$(Sgn(-2)) + " " + Hex$(Asc("a") - 200) + " " + Hex$(Len(s) - 9)
Print Hex$(0) + " " + Oct$(0) + " " + _Bin$(0) + " " + Hex$(48879) + " " + Oct$(8) + " " + _Bin$(5)
f = -2.5: d = 2.5: x = 1E+30
Print Hex$(f) + " " + Hex$(d) + " " + Hex$(-1.5) + " " + Hex$(3.5#) + " " + Oct$(f) + " " + _Bin$(d)
Print "["; Hex$(x); "]"
Print "["; Oct$(x); "]"
Print "["; _Bin$(x); "]"
Print Hex$(2## ^ 62) + " " + Hex$(-(2## ^ 62))
System

h:
Print "[handler"; Err; "]"
Resume Next
