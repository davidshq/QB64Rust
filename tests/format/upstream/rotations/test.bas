$Console:Only
Option _Explicit

Dim a As _Unsigned _Byte
Dim b As _Unsigned Integer
Dim c As _Unsigned Long
Dim d As _Unsigned _Integer64

a = &B11110000
b = &B1111111100000000
c = &B11111111111111110000000000000000
d = &B1111111111111111111111111111111100000000000000000000000000000000

Print "*** Unsigned values ***"
Print a; ":"; _Bin$(a)
Print b; ":"; _Bin$(b)
Print c; ":"; _Bin$(c)
Print d; ":"; _Bin$(d)

Print "Rotating left by 5"
Print _RoL(a, 1); ":"; _Bin$(_RoL(a, 5))
Print _RoL(b, 1); ":"; _Bin$(_RoL(b, 5))
Print _RoL(c, 1); ":"; _Bin$(_RoL(c, 5))
Print _RoL(d, 1); ":"; _Bin$(_RoL(d, 5))

Print "Rotating right by 5"
Print _RoR(a, 1); ":"; _Bin$(_RoR(a, 5))
Print _RoR(b, 1); ":"; _Bin$(_RoR(b, 5))
Print _RoR(c, 1); ":"; _Bin$(_RoR(c, 5))
Print _RoR(d, 1); ":"; _Bin$(_RoR(d, 5))

Print "Rotating left past size"
Print _RoL(a, 1); ":"; _Bin$(_RoL(a, 9))
Print _RoL(b, 1); ":"; _Bin$(_RoL(b, 17))
Print _RoL(c, 1); ":"; _Bin$(_RoL(c, 33))
Print _RoL(d, 1); ":"; _Bin$(_RoL(d, 65))

Print "Rotating right past size"
Print _RoR(a, 1); ":"; _Bin$(_RoR(a, 9))
Print _RoR(b, 1); ":"; _Bin$(_RoR(b, 17))
Print _RoR(c, 1); ":"; _Bin$(_RoR(c, 33))
Print _RoR(d, 1); ":"; _Bin$(_RoR(d, 65))

Print "Rotating left by -5"
Print _RoL(a, 1); ":"; _Bin$(_RoL(a, -5))
Print _RoL(b, 1); ":"; _Bin$(_RoL(b, -5))
Print _RoL(c, 1); ":"; _Bin$(_RoL(c, -5))
Print _RoL(d, 1); ":"; _Bin$(_RoL(d, -5))

Print "Rotating right by -5"
Print _RoR(a, 1); ":"; _Bin$(_RoR(a, -5))
Print _RoR(b, 1); ":"; _Bin$(_RoR(b, -5))
Print _RoR(c, 1); ":"; _Bin$(_RoR(c, -5))
Print _RoR(d, 1); ":"; _Bin$(_RoR(d, -5))


Dim e As _Byte
Dim f As Integer
Dim g As Long
Dim h As _Integer64

e = -128
f = -32768
g = -2147483648
h = -9223372036854775808

Print "*** Signed values ***"
Print e; ":"; _Bin$(e)
Print f; ":"; _Bin$(f)
Print g; ":"; _Bin$(g)
Print h; ":"; _Bin$(h)

Print "Rotating left by 5"
Print _RoL(e, 1); ":"; _Bin$(_RoL(e, 5))
Print _RoL(f, 1); ":"; _Bin$(_RoL(f, 5))
Print _RoL(g, 1); ":"; _Bin$(_RoL(g, 5))
Print _RoL(h, 1); ":"; _Bin$(_RoL(h, 5))

Print "Rotating right by 5"
Print _RoR(e, 1); ":"; _Bin$(_RoR(e, 5))
Print _RoR(f, 1); ":"; _Bin$(_RoR(f, 5))
Print _RoR(g, 1); ":"; _Bin$(_RoR(g, 5))
Print _RoR(h, 1); ":"; _Bin$(_RoR(h, 5))

Print "Rotating left past size"
Print _RoL(e, 1); ":"; _Bin$(_RoL(e, 9))
Print _RoL(f, 1); ":"; _Bin$(_RoL(f, 17))
Print _RoL(g, 1); ":"; _Bin$(_RoL(g, 33))
Print _RoL(h, 1); ":"; _Bin$(_RoL(h, 65))

Print "Rotating right past size"
Print _RoR(e, 1); ":"; _Bin$(_RoR(e, 9))
Print _RoR(f, 1); ":"; _Bin$(_RoR(f, 17))
Print _RoR(g, 1); ":"; _Bin$(_RoR(g, 33))
Print _RoR(h, 1); ":"; _Bin$(_RoR(h, 65))

Print "Rotating left by -5"
Print _RoL(e, 1); ":"; _Bin$(_RoL(e, -5))
Print _RoL(f, 1); ":"; _Bin$(_RoL(f, -5))
Print _RoL(g, 1); ":"; _Bin$(_RoL(g, -5))
Print _RoL(h, 1); ":"; _Bin$(_RoL(h, -5))

Print "Rotating right by -5"
Print _RoR(e, 1); ":"; _Bin$(_RoR(e, -5))
Print _RoR(f, 1); ":"; _Bin$(_RoR(f, -5))
Print _RoR(g, 1); ":"; _Bin$(_RoR(g, -5))
Print _RoR(h, 1); ":"; _Bin$(_RoR(h, -5))


Print "*** Rotating some numeric literals left by 5 ***"
Print 240~%%; ":"; _Bin$(_RoL(240~%%, 5))
Print 65280~%; ":"; _Bin$(_RoL(65280~%, 5))
Print 4294901760~&; ":"; _Bin$(_RoL(4294901760~&, 5))
Print 18446744069414584320~&&; ":"; _Bin$(_RoL(18446744069414584320~&&, 5))

Print "*** Rotating some numeric literals right by 5 ***"
Print 240~%%; ":"; _Bin$(_RoR(240~%%, 5))
Print 65280~%; ":"; _Bin$(_RoR(65280~%, 5))
Print 4294901760~&; ":"; _Bin$(_RoR(4294901760~&, 5))
Print 18446744069414584320~&&; ":"; _Bin$(_RoR(18446744069414584320~&&, 5))

System

