$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T224
    nums(0 TO 2) _Dynamic As _Unsigned Integer
End Type

Dim ok As _Byte
ReDim work(0) As T224

work(0).nums(0) = 0
work(0).nums(1) = 60000
work(0).nums(2) = 65535
ok = (work(0).nums(0) = 0 And work(0).nums(1) = 60000 And work(0).nums(2) = 65535)
If ok Then Print "PASS t224_unsigned_integer_dynamic" Else Print "FAIL t224_unsigned_integer_dynamic"
System
