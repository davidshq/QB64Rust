$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T225
    nums(0 TO 3) _Dynamic As Long
End Type

Dim ok As _Byte
Dim totalVal As Long
ReDim work(0) As T225

work(0).nums(0) = -100000
work(0).nums(1) = 1000
work(0).nums(2) = 2000
work(0).nums(3) = 97000
totalVal = work(0).nums(0) + work(0).nums(1) + work(0).nums(2) + work(0).nums(3)
ok = (totalVal = 0)
If ok Then Print "PASS t225_long_dynamic_arithmetic" Else Print "FAIL t225_long_dynamic_arithmetic"
System
