$Console:Only
' DECLARE LIBRARY with a header (v37h.h in this program's .files folder): every numeric type by value
' (BYVAL) and by reference, the conversion of a BYVAL argument, and what a parameter without BYVAL
' receives for each kind of argument (verification\v37_b, v37_c, x56, x58, x59, x63).
DefLng H
Const zero = 0
Declare Library "v37h"
    Function h_b%% (ByVal v As _Byte)
    Sub h_pb (v As _Byte)
    Function h_ub~%% (ByVal v As _Unsigned _Byte)
    Sub h_pub (v As _Unsigned _Byte)
    Function h_i% (ByVal v As Integer)
    Sub h_pi (v As Integer)
    Function h_ui~% (ByVal v As _Unsigned Integer)
    Sub h_pui (v As _Unsigned Integer)
    Function h_l& (ByVal v As Long)
    Sub h_pl (v As Long)
    Function h_ul~& (ByVal v As _Unsigned Long)
    Sub h_pul (v As _Unsigned Long)
    Function h_ll&& (ByVal v As _Integer64)
    Sub h_pll (v As _Integer64)
    Function h_ull~&& (ByVal v As _Unsigned _Integer64)
    Sub h_pull (v As _Unsigned _Integer64)
    Function h_s! (ByVal v As Single)
    Sub h_ps (v As Single)
    Function h_d# (ByVal v As Double)
    Sub h_pd (v As Double)
    Function h_f## (ByVal v As _Float)
    Sub h_pf (v As _Float)
    Function h_o%& (ByVal v As _Offset)
    Sub h_po (v As _Offset)
    Function h_uo~%& (ByVal v As _Unsigned _Offset)
    Sub h_puo (v As _Unsigned _Offset)
End Declare
Declare Library "v37h"
    Function h_isnull& (v As Long)
    Function h_isnull_o& Alias h_isnull (ByVal v As _Offset)
    Function h_peekl& (ByVal p As _Offset)
    Function h_add& (ByVal a As Long, ByVal b As Long)
    Sub h_tick
    Function h_ticks&
    Function h_dl Alias h_d (ByVal v As Double)
End Declare
Type t
    a As Long
    b As Long
End Type

Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, ui As _Unsigned Integer
Dim l As Long, ul As _Unsigned Long, ll As _Integer64, ull As _Unsigned _Integer64
Dim s As Single, d As Double, f As _Float, o As _Offset, uo As _Unsigned _Offset

Print "byval:"; h_b%%(126); h_ub~%%(254); h_i%(32766); h_ui~%(65534); h_l&(-5); h_ul~&(4294967294)
Print "byval:"; h_ll&&(-9); h_ull~&&(18446744073709551614~&&); h_s!(1.5); h_d#(2.25); h_f##(3.125)
Print "byval offset:"; h_o%&(41); h_uo~%&(41)
Print "wrap:"; h_b%%(127); h_ub~%%(255); h_i%(32767); h_ui~%(65535); h_l&(2147483647)

b = 1: ub = 2: i = 3: ui = 4: l = 5: ul = 6: ll = 7: ull = 8: s = 9.5: d = 10.5: f = 11.5: o = 12: uo = 13
h_pb b: h_pub ub: h_pi i: h_pui ui: h_pl l: h_pul ul: h_pll ll: h_pull ull
h_ps s: h_pd d: h_pf f: h_po o: h_puo uo
Print "byref:"; b; ub; i; ui; l; ul; ll; ull; s; d; f; o; uo

' A BYVAL argument of another type is converted as for any call.
Print "convert:"; h_l&(2.6); h_l&(3.5); h_s!(7); h_d#(1 / 3)
' Results used in expressions; a result without a suffix under DEFLNG is truncated by its cast.
Print "expr:"; h_l&(1) + h_l&(2) * 10; Len(Str$(h_d#(0.5))); h_dl(1.5); h_dl(1.7)

' What a parameter without BYVAL receives.
Dim x As Long, a(3) As Long, v As t, k As Integer, sg As Single
x = 10
h_pl x: Print "variable:"; x
h_pl (x): Print "parenthesised:"; x
h_pl x + 0: Print "expression:"; x
a(2) = 20: h_pl a(2): Print "element:"; a(2)
v.b = 30: h_pl v.b: Print "member:"; v.b
h_pl 5: Print "literal: done"
k = 40: h_pl k: Print "other type:"; k
sg = 1.5: h_ps sg: Print "single:"; sg
Call h_pl(x): Print "call:"; x

' A BYVAL argument written 0 is NULL.
Print "null literal 0:"; h_isnull&(0); " null 1:"; h_isnull&(1); " null var:"; h_isnull&(x)
Print "byval offset 0:"; h_isnull_o&(0); h_isnull_o&((0)); h_isnull_o&(0&); h_isnull_o&(zero)
x = 1234: Print "peek:"; h_peekl&(_Offset(x))
Print "add:"; h_add&(2, 3)
h_tick: h_tick: Call h_tick
Print "ticks:"; h_ticks&
inner
System

Sub inner
    ' A block in a SUB; its names are global.
    Declare Library "v37h"
        Function h_add2& Alias h_add (ByVal a As Long, ByVal b As Long)
    End Declare
    Print "sub:"; h_add2&(20, 22); h_ticks&
End Sub
