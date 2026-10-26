$Console:Only

Option _Explicit

Const TEXT_HELLO = "hello"
Const TEXT_WORLD = "world"

Type t
    f As Single
    i As _Offset
End Type

Dim f As Single: f = 3.141592653589793239
Dim i As _Offset: i = 255
Dim s As String: s = TEXT_HELLO

Dim t As t
t.f = f
t.i = i

Dim a(1) As Single
a(0) = f
a(1) = _IIf(_FALSE, 0, i) ' naughty!

Print _IIf(f > 0, f, i)
Print _IIf(i > 0, i, f)

Print _IIf(t.f > 0, t.f, t.i)
Print _IIf(t.i > 0, t.i, t.f)

Print _IIf(a(0) > 0, a(0), a(1))
Print _IIf(a(1) > 0, a(1), a(0))

Print _IIf(f > 0, 1.1!, 2)
Print _IIf(i > 0, 2, 1.1!)

Print _IIf(f = 0, "f", "i")
Print _IIf(i = 0, "i", "f")

' PRINT and LPRINT has bugs and throws error when using string comparisons
Write _IIf(s = TEXT_HELLO, TEXT_HELLO, TEXT_WORLD)
Write _IIf(s = TEXT_WORLD, 1, 2)

Print _IIf(i > 0, foo, bar)
Print _IIf(i < 0, bar, foo)

Dim age As _Byte: age = 17
Print _IIf(age >= 18, "over 18", "under 18")

System

Function foo&
    Print "foo called!"
    foo = 512
End Function

Function bar&
    Print "bar called!"
    bar = 128
End Function
