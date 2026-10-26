$Console:Only
$Unstable:TypeFields

Option _Explicit

Type P245
    xVal As Long
    yVal As Long
End Type

Type T245
    pts(0 TO 1, 0 TO 1) _Dynamic As P245
End Type

Dim ok As _Byte
ReDim work(0) As T245

work(0).pts(0, 0).xVal = 1
work(0).pts(0, 0).yVal = 2
work(0).pts(1, 1).xVal = 3
work(0).pts(1, 1).yVal = 4
ok = (work(0).pts(0, 0).xVal = 1 And work(0).pts(0, 0).yVal = 2 And work(0).pts(1, 1).xVal = 3 And work(0).pts(1, 1).yVal = 4)
If ok Then Print "PASS t245_nested_fixed_udt_2d_dynamic_field" Else Print "FAIL t245_nested_fixed_udt_2d_dynamic_field"
System
