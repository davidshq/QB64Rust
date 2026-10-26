$Console:Only
$Unstable:TypeFields

Option _Explicit

Type P244
    xVal As Long
    yVal As Long
End Type

Type T244
    pts(1 TO 2) _Dynamic As P244
End Type

Dim ok As _Byte
ReDim work(0) As T244

work(0).pts(1).xVal = 10
work(0).pts(1).yVal = 20
work(0).pts(2).xVal = 30
work(0).pts(2).yVal = 40
ok = (work(0).pts(1).xVal + work(0).pts(1).yVal = 30 And work(0).pts(2).xVal + work(0).pts(2).yVal = 70)
If ok Then Print "PASS t244_nested_fixed_udt_dynamic_field" Else Print "FAIL t244_nested_fixed_udt_dynamic_field"
System
