$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T228
    nums(0 TO 2) _Dynamic As _Unsigned _Integer64
End Type

Dim ok As _Byte
ReDim work(0) As T228

work(0).nums(0) = 1000000000
work(0).nums(1) = 2000000000
work(0).nums(2) = 3000000000
ok = (work(0).nums(0) + work(0).nums(1) = work(0).nums(2))
If ok Then Print "PASS t228_unsigned_integer64_dynamic" Else Print "FAIL t228_unsigned_integer64_dynamic"
System
