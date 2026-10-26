$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T221
    nums(0 TO 2) _Dynamic As _Byte
End Type

Dim ok As _Byte
ReDim work(0) As T221

work(0).nums(0) = -120
work(0).nums(1) = 0
work(0).nums(2) = 120
ok = (work(0).nums(0) = -120 And work(0).nums(1) = 0 And work(0).nums(2) = 120)
If ok Then Print "PASS t221_signed_byte_dynamic" Else Print "FAIL t221_signed_byte_dynamic"
System
