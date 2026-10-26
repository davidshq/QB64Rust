$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T230
    nums(0 TO 2) _Dynamic As Double
End Type

Dim ok As _Byte
ReDim work(0) As T230

work(0).nums(0) = 1.125#
work(0).nums(1) = 2.25#
work(0).nums(2) = work(0).nums(0) * work(0).nums(1)
ok = (Abs(work(0).nums(2) - 2.53125#) < .0000001#)
If ok Then Print "PASS t230_double_dynamic_tolerance" Else Print "FAIL t230_double_dynamic_tolerance"
System
