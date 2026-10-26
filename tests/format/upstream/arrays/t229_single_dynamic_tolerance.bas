$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T229
    nums(0 TO 2) _Dynamic As Single
End Type

Dim ok As _Byte
ReDim work(0) As T229

work(0).nums(0) = 1.25
work(0).nums(1) = 2.5
work(0).nums(2) = work(0).nums(0) + work(0).nums(1)
ok = (Abs(work(0).nums(2) - 3.75) < .0001)
If ok Then Print "PASS t229_single_dynamic_tolerance" Else Print "FAIL t229_single_dynamic_tolerance"
System
