$Console:Only
' Slice program (m2-numeric-types, task 5.4): the new numeric types' declarations (every AS spelling and suffix),
' literals (held as written, believed the suffix's type), LEN, stores, and the scenarios of the numeric-semantics
' spec delta that the old compiler agrees with (not the decided divergences D-005, D-006, D-009, nor the compile
' errors; HEX$ of a new type comes with task 8.4). No PRINT comma.
Dim a As _Byte, b As _Unsigned _Byte, c As Integer, d As _Unsigned Integer, e As Long, f As _Unsigned Long
Dim g As _Integer64, h As _Unsigned _Integer64, o As _Offset, uo As _Unsigned _Offset
Dim t As _Bit, ut As _Unsigned _Bit, t7 As _Bit * 7, ut9 As _Unsigned _Bit * 9, s As _Unsigned String
Print Len(a); Len(b); Len(c); Len(d); Len(e); Len(f); Len(g); Len(h); Len(o); Len(uo)
' Stores wrap to the width and signedness.
a = 200: b = 300: c = 70000: d = -1: e = 3000000000: f = -1: g = -1: h = -1: o = -1: uo = -1
Print a; b; c; d; e; f; g; h; o; uo
t = 1: ut = 3: t7 = 100: ut9 = 1000: s = "plain"
Print t; ut; t7; ut9; s
' Suffixes: one name, a variable per suffix.
n%% = 1: n~%% = 2: n~% = 3: n~& = 4: n~&& = 5: n%& = 6: n~%& = 7: n` = 1: n`5 = 9: n~` = 1: n~`5 = 33
Print n%%; n~%%; n~%; n~&; n~&&; n%&; n~%&; n`; n`5; n~`; n~`5
Print Len(n%%); Len(n~%&)
' Scenarios: numeric types.
Dim ub As _Unsigned _Byte: ub = 255: Print ub; Len(ub)
x~% = 65535: Print x~%
' A pad before the _BIT * 64, which the old compiler would let overwrite the _BIT before it (D-009).
Dim pad As _Bit * 32, u64 As _Unsigned _Bit * 64: u64 = -1: Print u64
' Arithmetic with narrow operands.
b~%% = 255: Print b~%% + 1; x~% * 2
' _BIT stores: masked or sign-extended; in arithmetic a 32-bit storage.
Dim u3 As _Unsigned _Bit * 3, b3 As _Bit * 3, one As _Bit
u3 = 7: Print u3 * 1000000000; u3 > -1
b3 = 5: Print b3
u3 = 13: Print u3
one = 1: Print one
' Literals.
Print -3; 40000; -2147483648; 4294967295~&; 255~%%; &HFF%%; &HFF%% + 0
Print 300~%%; -1~&; 40000%; 300~%% + 0; -1~& + 0
l& = 300~%%: Print l&
Print 4294967295~& + 1; -1~&& + 0; -1~&& > 0; 255~%% = -1%%
Print 1`; 9`3; 9`3 + 0
Print &H1FF~%%; &H1FF~%% + 0
' Stores from floats: half to even; 16 bits or fewer from SINGLE, wider and _BIT from _FLOAT.
x% = 2.5: Print x%;: x% = 3.5: Print x%
dd# = 2.5000001: x% = dd#: l& = dd#: Print x%; l&
Dim b16 As _Bit * 16: u~% = dd#: b16 = dd#: Print u~%; b16
x% = 70000: Print x%
x~% = -1: u~&& = -1: Print x~%; u~&&
b%% = 200: Print b%%
' Logical operators and integer division (old types; unchanged).
Print 0.4 _AndAlso 1; _Negate 0.4
Print 1.5 And 3; 2.5 Or 0; Not 1.5; 5 Xor 3; 5 Eqv 3; 5 Imp 3
Print 1 _AndAlso 2; 0 _OrElse 0; _Negate 0; _Negate 5
Print 7 \ 2; -7 \ 2; 7.5 \ 2; -7 Mod 3; 7.5 Mod 2
' _OFFSET division keeps the type.
uo = 7: Print uo / 2
System
