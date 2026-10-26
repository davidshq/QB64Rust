$Console:Only

Dim dim_m As _MEM

Dim dim_s As String
Dim dim_s2 As String * 8

Dim dim_b As _Bit
Dim dim_b2 As _Unsigned _Bit

Dim dim_bn As _Bit * 7
Dim dim_bn2 As _Unsigned _Bit * 7

Dim dim_byte As _Byte
Dim dim_byte2 As _Unsigned _Byte

Dim dim_i64 As _Integer64
Dim dim_i64_2 As _Unsigned _Integer64

Dim dim_o As _Offset
Dim dim_o2 As _Unsigned _Offset

Dim dim_i As Integer
Dim dim_i2 As _Unsigned Integer

Dim dim_l As Long
Dim dim_l2 As _Unsigned Long

Dim dim_si As Single
Dim dim_d As Double
Dim dim_f As _Float

test

System

Sub test ()
    Static m As _MEM

    Static s As String
    Static s2 As String * 8

    Static b As _Bit
    Static b2 As _Unsigned _Bit

    Static bn As _Bit * 7
    Static bn2 As _Unsigned _Bit * 7

    Static byt As _Byte
    Static byt2 As _Unsigned _Byte

    Static i64 As _Integer64
    Static i64_2 As _Unsigned _Integer64

    Static o As _Offset
    Static o2 As _Unsigned _Offset

    Static i As Integer
    Static i2 As _Unsigned Integer

    Static l As Long
    Static l2 As _Unsigned Long

    Static si As Single
    Static d As Double
    Static f As _Float

    ' Just check a few of them
    s2 = "HI"
    b = -1
    bn = 20
    byt = 50
    i64 = 23456
    i = 70
    l = 80
    si = 2.2
    d = 2.4
    f = 2.8

    Print "TEST: "; s2; b; bn; byt; i64; i; l; si; d; f
End Sub

