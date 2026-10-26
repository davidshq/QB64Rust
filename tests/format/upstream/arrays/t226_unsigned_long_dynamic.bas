$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T226
    nums(0 TO 2) _Dynamic As _Unsigned Long
End Type

Dim ok As _Byte
ReDim work(0) As T226

work(0).nums(0) = 123456789
work(0).nums(1) = 987654321
work(0).nums(2) = 1111111110
ok = (work(0).nums(0) + work(0).nums(1) = work(0).nums(2))
If ok Then Print "PASS t226_unsigned_long_dynamic" Else Print "FAIL t226_unsigned_long_dynamic"
System
