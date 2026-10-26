$Console:Only

Option _Explicit

Const CF! = 3.141592653589793239
Const CI%& = 255

Type t
    f As Single
    i As _Offset
End Type

Dim f As Single: f = CF
Dim i As _Offset: i = CI

Dim t As t
t.f = f
t.i = i

Dim a(1) As Single
a(0) = f
a(1) = _Cast(Single, i)

Print _Cast(_Byte, CF)
Print _Cast(_Byte, CI)
Print _Cast(_Byte, f * 2)
Print _Cast(_Byte, i * 2)
Print _Cast(_Byte, t.f)
Print _Cast(_Byte, t.i)
Print _Cast(_Byte, a(0) * 2)
Print _Cast(_Byte, a(1) * 2)

Print _Cast(_Unsigned _Byte, CF)
Print _Cast(_Unsigned _Byte, CI)
Print _Cast(_Unsigned _Byte, f * 2)
Print _Cast(_Unsigned _Byte, i * 2)
Print _Cast(_Unsigned _Byte, t.f)
Print _Cast(_Unsigned _Byte, t.i)
Print _Cast(_Unsigned _Byte, a(0) * 2)
Print _Cast(_Unsigned _Byte, a(1) * 2)

Print _Cast(Integer, CF)
Print _Cast(Integer, CI)
Print _Cast(Integer, f * 2)
Print _Cast(Integer, i * 2)
Print _Cast(Integer, t.f)
Print _Cast(Integer, t.i)
Print _Cast(Integer, a(0) * 2)
Print _Cast(Integer, a(1) * 2)

Print _Cast(_Unsigned Integer, CF)
Print _Cast(_Unsigned Integer, CI)
Print _Cast(_Unsigned Integer, f * 2)
Print _Cast(_Unsigned Integer, i * 2)
Print _Cast(_Unsigned Integer, t.f)
Print _Cast(_Unsigned Integer, t.i)
Print _Cast(_Unsigned Integer, a(0) * 2)
Print _Cast(_Unsigned Integer, a(1) * 2)

Print _Cast(Long, CF)
Print _Cast(Long, CI)
Print _Cast(Long, f * 2)
Print _Cast(Long, i * 2)
Print _Cast(Long, t.f)
Print _Cast(Long, t.i)
Print _Cast(Long, a(0) * 2)
Print _Cast(Long, a(1) * 2)

Print _Cast(_Unsigned Long, CF)
Print _Cast(_Unsigned Long, CI)
Print _Cast(_Unsigned Long, f * 2)
Print _Cast(_Unsigned Long, i * 2)
Print _Cast(_Unsigned Long, t.f)
Print _Cast(_Unsigned Long, t.i)
Print _Cast(_Unsigned Long, a(0) * 2)
Print _Cast(_Unsigned Long, a(1) * 2)

Print _Cast(_Integer64, CF)
Print _Cast(_Integer64, CI)
Print _Cast(_Integer64, f * 2)
Print _Cast(_Integer64, i * 2)
Print _Cast(_Integer64, t.f)
Print _Cast(_Integer64, t.i)
Print _Cast(_Integer64, a(0) * 2)
Print _Cast(_Integer64, a(1) * 2)

Print _Cast(_Unsigned _Integer64, CF)
Print _Cast(_Unsigned _Integer64, CI)
Print _Cast(_Unsigned _Integer64, f * 2)
Print _Cast(_Unsigned _Integer64, i * 2)
Print _Cast(_Unsigned _Integer64, t.f)
Print _Cast(_Unsigned _Integer64, t.i)
Print _Cast(_Unsigned _Integer64, a(0) * 2)
Print _Cast(_Unsigned _Integer64, a(1) * 2)

Print _Cast(_Offset, CF)
Print _Cast(_Offset, CI)
Print _Cast(_Offset, f * 2)
Print _Cast(_Offset, i * 2)
Print _Cast(_Offset, t.f)
Print _Cast(_Offset, t.i)
Print _Cast(_Offset, a(0) * 2)
Print _Cast(_Offset, a(1) * 2)

Print _Cast(_Unsigned _Offset, CF)
Print _Cast(_Unsigned _Offset, CI)
Print _Cast(_Unsigned _Offset, f * 2)
Print _Cast(_Unsigned _Offset, i * 2)
Print _Cast(_Unsigned _Offset, t.f)
Print _Cast(_Unsigned _Offset, t.i)
Print _Cast(_Unsigned _Offset, a(0) * 2)
Print _Cast(_Unsigned _Offset, a(1) * 2)

Print Using "#####.###"; _Cast(Single, CF)
Print Using "#####.###"; _Cast(Single, CI)
Print Using "#####.###"; _Cast(Single, f * 2)
Print Using "#####.###"; _Cast(Single, i * 2)
Print Using "#####.###"; _Cast(Single, t.f)
Print Using "#####.###"; _Cast(Single, t.i)
Print Using "#####.###"; _Cast(Single, a(0) * 2)
Print Using "#####.###"; _Cast(Single, a(1) * 2)

Print Using "#####.###"; _Cast(Double, CF)
Print Using "#####.###"; _Cast(Double, CI)
Print Using "#####.###"; _Cast(Double, f * 2)
Print Using "#####.###"; _Cast(Double, i * 2)
Print Using "#####.###"; _Cast(Double, t.f)
Print Using "#####.###"; _Cast(Double, t.i)
Print Using "#####.###"; _Cast(Double, a(0) * 2)
Print Using "#####.###"; _Cast(Double, a(1) * 2)

Print Using "#####.###"; _Cast(_Float, CF)
Print Using "#####.###"; _Cast(_Float, CI)
Print Using "#####.###"; _Cast(_Float, f * 2)
Print Using "#####.###"; _Cast(_Float, i * 2)
Print Using "#####.###"; _Cast(_Float, t.f)
Print Using "#####.###"; _Cast(_Float, t.i)
Print Using "#####.###"; _Cast(_Float, a(0) * 2)
Print Using "#####.###"; _Cast(_Float, a(1) * 2)

System
