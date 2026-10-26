$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T227
    nums(0 TO 2) _Dynamic As _Integer64
End Type

Dim ok As _Byte
ReDim work(0) As T227

work(0).nums(0) = -2000000000
work(0).nums(1) = 3000000000
work(0).nums(2) = 1000000000
ok = (work(0).nums(0) + work(0).nums(1) = work(0).nums(2))
If ok Then Print "PASS t227_integer64_dynamic" Else Print "FAIL t227_integer64_dynamic"
System
