$Console:Only
' Slice program (m2-numeric-types, task 6.1): stores into _BIT and _BIT * n scalars (the mask for unsigned ones,
' the sign extension from bit n-1 for signed ones) at each width, from integers and floats; _BIT variables in
' each storage class; a _BIT variable passed to an integer parameter (always a copy); _BIT values in arithmetic,
' comparisons and STR$. Every _BIT * n wider than 32 bits is declared after a _BIT * 32 pad, so the old compiler's
' overlap (DIVERGENCES.md D-009) hits nothing printed; _BIT arrays are not supported yet. No PRINT comma.
Dim Shared sh As _Bit * 4
Dim b1 As _Bit, u1 As _Unsigned _Bit, b3 As _Bit * 3, u3 As _Unsigned _Bit * 3
Dim b7 As _Bit * 7, u7 As _Unsigned _Bit * 7, b16 As _Bit * 16, u16 As _Unsigned _Bit * 16
Dim b17 As _Bit * 17, u20 As _Unsigned _Bit * 20, b31 As _Bit * 31, u31 As _Unsigned _Bit * 31
Dim b32 As _Bit * 32, u32 As _Unsigned _Bit * 32
Dim pad1 As _Bit * 32, b33 As _Bit * 33
Dim pad2 As _Bit * 32, u33 As _Unsigned _Bit * 33
Dim pad3 As _Bit * 32, b40 As _Bit * 40
Dim pad4 As _Bit * 32, u40 As _Unsigned _Bit * 40
Dim pad5 As _Bit * 32, b63 As _Bit * 63
Dim pad6 As _Bit * 32, b64 As _Bit * 64
Dim pad7 As _Bit * 32, u64 As _Unsigned _Bit * 64
Dim q As _Integer64, ul As _Unsigned Long, d As Double, fl As _Float
' Integer stores: the low n bits, then the sign.
b1 = 1: Print "b1:"; b1;: b1 = 2: Print b1;: b1 = 3: Print b1;: b1 = -2: Print b1
u1 = 1: Print "u1:"; u1;: u1 = 2: Print u1;: u1 = -1: Print u1
b3 = 3: Print "b3:"; b3;: b3 = 4: Print b3;: b3 = 5: Print b3;: b3 = 8: Print b3;: b3 = -5: Print b3;: b3 = 13: Print b3
u3 = 7: Print "u3:"; u3;: u3 = 8: Print u3;: u3 = 13: Print u3;: u3 = -1: Print u3
b7 = 63: Print "b7:"; b7;: b7 = 64: Print b7;: b7 = 200: Print b7;: b7 = -129: Print b7
u7 = 127: Print "u7:"; u7;: u7 = 128: Print u7;: u7 = -1: Print u7
b16 = 32767: Print "b16:"; b16;: b16 = 32768: Print b16;: b16 = 65535: Print b16;: b16 = 70000: Print b16
u16 = 65535: Print "u16:"; u16;: u16 = 65536: Print u16;: u16 = -1: Print u16
b17 = 65536: Print "b17:"; b17;: b17 = 131071: Print b17;: b17 = -65537: Print b17
u20 = 1048575: Print "u20:"; u20;: u20 = 1048576: Print u20;: u20 = -2: Print u20
b31 = 1073741823: Print "b31:"; b31;: b31 = 1073741824: Print b31;: b31 = -1: Print b31
u31 = 2147483647: Print "u31:"; u31;: u31 = -1: Print u31
b32 = 2147483647: Print "b32:"; b32;: b32 = 4294967295: Print b32;: b32 = 4294967296: Print b32
u32 = 4294967295: Print "u32:"; u32;: u32 = -1: Print u32;: u32 = 4294967298: Print u32
b33 = 4294967295: Print "b33:"; b33;: b33 = 8589934591: Print b33;: b33 = -4294967297: Print b33
u33 = 8589934591: Print "u33:"; u33;: u33 = -1: Print u33
b40 = 549755813887: Print "b40:"; b40;: b40 = 549755813888: Print b40;: b40 = 1E+12: Print b40
u40 = -1: Print "u40:"; u40;: u40 = 1099511627776: Print u40
b63 = -1: Print "b63:"; b63;: b63 = 4611686018427387904: Print b63
b64 = -1: Print "b64:"; b64;: b64 = 9223372036854775807: Print b64
u64 = 18446744073709551615~&&: Print "u64:"; u64; u64 + 0; u64 > 0
' Stores from other integer types.
q = 4294967298: b7 = q: Print "from _INTEGER64:"; b7;: q = -129: b7 = q: Print b7
ul = 4294967295: b7 = ul: Print " from _UNSIGNED LONG:"; b7;: u3 = ul: Print u3
b3 = -3: u7 = b3: Print "from _BIT * 3:"; u7;: b16 = b3: Print b16
' Stores from floats: rounded half to even from _FLOAT, whatever the width, then masked.
b3 = 2.5: Print "floats:"; b3;: b3 = 3.5: Print b3;: b3 = -0.5: Print b3;: b3 = -1.5: Print b3
u3 = 6.5: Print u3;: u3 = 7.5: Print u3
d = 2.5000001: b16 = d: b17 = d: b31 = d: b32 = d: Print " 2.5000001#:"; b16; b17; b31; b32
d = 16777217.5#: b17 = d: b31 = d: b32 = d: Print " 16777217.5#:"; b17; b31; b32
d = 65537.5#: b17 = d: Print " 65537.5#:"; b17
u20 = -1.5: Print " u20:"; u20;: u20 = 1048575.5: Print u20
fl = 549755813887.5##: b40 = fl: Print " b40:"; b40;: u40 = -2.5: Print u40
u64 = 1.8E+19: Print " u64:"; u64
' Arithmetic: a _BIT * n of up to 32 bits counts as its 32-bit storage.
b3 = -3: u3 = 7
Print "arith:"; b3 + 1; b3 * b3; u3 + 1; u3 * 1000000000; b3 - u3; -b3; Not u3; b3 = -3; u3 > b3
u32 = 4294967295: Print "u32:"; u32 + 1; u32 * 2; u32 > 0; u32 > -1
b1 = -1: Print "b1:"; b1 + b1; Not b1; b1 And 5
If b1 Then Print "b1 is true"
Print "STR$: ["; Str$(b3); "] ["; Str$(u3); "] ["; Str$(u64); "]"
b3 = b3 + 2: Print "b3 + 2:"; b3;: b3 = b3 + 2: Print b3
' Storage classes.
sh = 9: Print "shared:"; sh
st
st
lo
lo
showsh
' Passed to integer parameters: always a copy, whatever the parameter's type.
Dim b5 As _Bit * 5, ub5 As _Unsigned _Bit * 5
b5 = -16: showsl b5: Print " after:"; b5
b5 = 7: showsi b5: Print " after:"; b5
ub5 = 31: showul ub5: Print " after:"; ub5
ub5 = 31: showsl ub5: Print " after:"; ub5
b40 = -1: showsq b40: Print " after:"; b40
System

Sub st
    Static b As _Bit * 4
    b = b + 5
    Print "static:"; b
End Sub

Sub lo
    Dim b As _Bit * 4, u As _Unsigned _Bit * 4
    Print "local:"; b; u;
    b = 7: u = 17
    Print b; u
End Sub

Sub showsh
    Print "shared in SUB:"; sh
End Sub

Sub showsi (x As Integer)
    Print "INTEGER param:"; x;: x = 99
End Sub
Sub showsl (x As Long)
    Print "LONG param:"; x;: x = 5
End Sub
Sub showul (x As _Unsigned Long)
    Print "_UNSIGNED LONG param:"; x;: x = 5
End Sub
Sub showsq (x As _Integer64)
    Print "_INTEGER64 param:"; x;: x = 6
End Sub
