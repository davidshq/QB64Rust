$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T223
    nums(0 TO 2) _Dynamic As Integer
End Type

Dim ok As _Byte
ReDim work(0) As T223

work(0).nums(0) = -30000
work(0).nums(1) = 123
work(0).nums(2) = 30000
ok = (work(0).nums(0) = -30000 And work(0).nums(1) = 123 And work(0).nums(2) = 30000)
If ok Then Print "PASS t223_integer_dynamic" Else Print "FAIL t223_integer_dynamic"
System
