$Console:Only

test

System

Sub test ()
    Static m(3) As _MEM

    Static s(3) As String
    Static s2(3) As String * 8

    Static b(3) As _Bit
    Static b2(3) As _Unsigned _Bit

    Static bn(3) As _Bit * 7
    Static bn2(3) As _Unsigned _Bit * 7

    Static byt(3) As _Byte
    Static byt2(3) As _Unsigned _Byte

    Static i64(3) As _Integer64
    Static i64_2(3) As _Unsigned _Integer64

    Static o(3) As _Offset
    Static o2(3) As _Unsigned _Offset

    Static i(3) As Integer
    Static i2(3) As _Unsigned Integer

    Static l(3) As Long
    Static l2(3) As _Unsigned Long

    Static si(3) As Single
    Static d(3) As Double
    Static f(3) As _Float

    ' Just check a few of them
    s2(1) = "HI"
    b(1) = -1
    bn(1) = 20
    byt(1) = 50
    i64(1) = 23456
    i(1) = 70
    l(1) = 80
    si(1) = 2.2
    d(1) = 2.4
    f(1) = 2.8

    Print "TEST: "; s2(1); b(1); bn(1); byt(1); i64(1); i(1); l(1); si(1); d(1); f(1)
End Sub

