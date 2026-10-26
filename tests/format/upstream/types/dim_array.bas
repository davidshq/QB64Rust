$Console:Only

Dim m(3) As _MEM

Dim s(3) As String
Dim s2(3) As String * 8

Dim b(3) As _Bit
Dim b2(3) As _Unsigned _Bit

Dim bn(3) As _Bit * 7
Dim bn2(3) As _Unsigned _Bit * 7

Dim byt(3) As _Byte
Dim byt2(3) As _Unsigned _Byte

Dim i64(3) As _Integer64
Dim i64_2(3) As _Unsigned _Integer64

Dim o(3) As _Offset
Dim o2(3) As _Unsigned _Offset

Dim i(3) As Integer
Dim i2(3) As _Unsigned Integer

Dim l(3) As Long
Dim l2(3) As _Unsigned Long

Dim si(3) As Single
Dim d(3) As Double
Dim f(3) As _Float

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

System

