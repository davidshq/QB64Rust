Option _Explicit
$Console:Only

' Most of tests are meant for testing few overloaded QB64-PE functions on ARM6 targets.
' However, these tests should not fail on other targets.

Print Val("") ' 0
Print Val("  42  ") ' 42
Print Val("-999") ' -999"
Print Val("bad123") ' 0
Print Val("123bad") ' 123
Print Val("123.456bad") ' 123.456
Print Val("123") ' 123
Print Val("123.456") ' 123.456
Print Val("123.456.789") ' 123.456
Print Val("123.456e2") ' 12345.6
Print Val("123.456e-2") ' 1.23456
Print Val("&H1A2B") ' 6699
Print Val("&B11110000") ' 240
Print Val("&O177") ' 127
Print Val("1d-1") ' 0.1
Print Val("1d2") ' 100
Print Val("1D-1") ' 0.1
Print Val("1D+2") ' 100
Print Val("1e-1") ' 0.1
Print Val("1e2") ' 100
Print Val("1E-1") ' 0.1
Print Val("1E+2") ' 100
Print Val("1f-1") ' 0.1
Print Val("1f2") ' 100
Print Val("1F-1") ' 0.1
Print Val("1F+2") ' 100
Print

' The following test the new behavior on ARM64 targets
Print Val("18446744073709551615", _Unsigned _Integer64) ' 18446744073709551615
Print Val("-9223372036854775808", _Integer64) ' -9223372036854775808
Print Val("9223372036854775807", _Integer64) ' 9223372036854775807
Print Val("-4611686018427387903", _Integer64) ' -4611686018427387903
Print Val("4611686018427387903", _Unsigned _Integer64) ' 4611686018427387903
Print

' The following tests the new behavior for _MIN, _MAX and _CLAMP on ARM64 targets
Print _Min(-9223372036854775808&&, 9223372036854775807&&) ' -9223372036854775808
Print _Max(-9223372036854775808&&, 9223372036854775807&&) ' 9223372036854775807
Print _Clamp(18446744073709551615~&&, 4611686018427387903&&, 0&&) ' 4611686018427387903
Print

' Different types of constants
Const B%% = 101%%
Const UB~%% = 201~%%
Const I% = 32001%
Const UI~% = 64001~%
Const L& = 2000000001&
Const UL~& = 4000000001~&
Const I64&& = 8000000000000000001&&
Const UI64~&& = 16000000000000000001~&&
Const S! = 3141592E-6 ' 3.141592
Const D# = 2718281D-6 ' 2.718281
Const F## = 1618033F-6 ' 1.618033
Const O&& = &O157255
Const UO~&& = &O137357

Type Bar
    b As _Byte
    ub As _Unsigned _Byte
    i As Integer
    ui As _Unsigned Integer
    l As Long
    ul As _Unsigned Long
    i64 As _Integer64
    ui64 As _Unsigned _Integer64
    s As Single
    d As Double
    f As _Float
    o As _Integer64
    uo As _Unsigned _Integer64
End Type

Dim Foo As Bar
Foo.b = B%%
Foo.ub = UB~%%
Foo.i = I%
Foo.ui = UI~%
Foo.l = L&
Foo.ul = UL~&
Foo.i64 = I64&&
Foo.ui64 = UI64~&&
Foo.s = S!
Foo.d = D#
Foo.f = F##
Foo.o = O%&
Foo.uo = UO~%&

Dim Baz(0 To 3) As Bar
Dim Arr(0 To 3) As Long

Dim n As Long
For n = 0 To 3
    Arr(n) = n + 1
    Baz(n).b = B
    Baz(n).ub = UB
    Baz(n).i = I
    Baz(n).ui = UI
    Baz(n).l = L
    Baz(n).ul = UL
    Baz(n).i64 = I64
    Baz(n).ui64 = UI64
    Baz(n).s = S
    Baz(n).d = D
    Baz(n).f = F
    Baz(n).o = O
    Baz(n).uo = UO
Next

' These help us check if the underlying type promotion logic for _MIN, _MAX, and _CLAMP works correctly

' Constant cases
Print _Min(B, UI) ' 101
Print _Min(I64, L) ' 2000000001
Print _Clamp(UI, UB, B) ' 201
Print _Min(S, UI) ' 3.141592
Print _Max(F, D) ' 2.718281
Print _Clamp(F, S, D) '  2.718281
Print _Min(UI, F) ' 1.618033
Print _Max(F, B) ' 101
Print _Clamp(F, B, S) ' 3.141592
Print

' Array of LONG cases
Print _Min(Arr(0), Arr(1)) ' 1
Print _Min(Arr(2), Arr(3)) ' 3
Print _Clamp(Arr(0), Arr(1), Arr(2)) ' 2
Print

' UDT cases
Print _Min(Foo.b, Foo.ub) ' 101
Print _Min(Foo.i64, Foo.l) ' 2000000001
Print _Clamp(Foo.ui, Foo.ub, Foo.b) ' 201
Print _Min(Foo.s, Foo.ui) ' 3.141592
Print _Max(Foo.f, Foo.d) ' 2.718281
Print _Clamp(Foo.f, Foo.s, Foo.d) ' 2.718281
Print _Min(Foo.ui, Foo.f) ' 1.618033
Print _Max(Foo.f, Foo.b) ' 101
Print Int(_Clamp(Foo.f, Foo.b, Foo.s)) ' 3
Print

' Array of UTD cases
Print _Min(Baz(0).b, Baz(1).ub) ' 101
Print _Min(Baz(2).i64, Baz(3).l) ' 2000000001
Print _Clamp(Baz(0).ui, Baz(1).ub, Baz(2).b) ' 201
Print _Min(Baz(3).s, Baz(0).ui) ' 3.141592
Print _Max(Baz(1).f, Baz(2).d) ' 2.718281
Print _Clamp(Baz(1).f, Baz(3).s, Baz(2).d) ' 2.718281
Print _Min(Baz(0).ui, Baz(1).f) ' 1.618033
Print _Max(Baz(1).f, Baz(2).b) ' 101
Print Int(_Clamp(Baz(1).f, Baz(2).b, Baz(3).s)) ' 3
Print Hex$(_Max(Baz(0).o, Baz(1).uo)) ' DEAD

System
