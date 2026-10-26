$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T222
    nums(0 TO 2) _Dynamic As _Unsigned _Byte
End Type

Dim ok As _Byte
ReDim work(0) As T222

work(0).nums(0) = 0
work(0).nums(1) = 250
work(0).nums(2) = 255
ok = (work(0).nums(0) = 0 And work(0).nums(1) = 250 And work(0).nums(2) = 255)
If ok Then Print "PASS t222_unsigned_byte_dynamic" Else Print "FAIL t222_unsigned_byte_dynamic"
System
