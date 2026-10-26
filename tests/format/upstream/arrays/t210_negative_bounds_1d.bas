$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T210
    nums(-2 TO 2) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T210

work(0).nums(-2) = 10
work(0).nums(-1) = 20
work(0).nums(0) = 30
work(0).nums(1) = 40
work(0).nums(2) = 50

ok = (LBound(work(0).nums) = -2 And UBound(work(0).nums) = 2 And work(0).nums(-2) = 10 And work(0).nums(2) = 50)
If ok Then Print "PASS t210_negative_bounds_1d" Else Print "FAIL t210_negative_bounds_1d"
System
