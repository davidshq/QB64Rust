$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T233
    nums(0 TO 2) As Long
End Type

Dim ok As _Byte
ReDim work(0) As T233

work(0).nums(0) = 10
work(0).nums(1) = 20
work(0).nums(2) = 30
ok = (work(0).nums(0) + work(0).nums(1) + work(0).nums(2) = 60)
If ok Then Print "PASS t233_unmarked_inline_numeric_array_redim_parent" Else Print "FAIL t233_unmarked_inline_numeric_array_redim_parent"
System
