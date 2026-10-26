$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T214
    nums(1 TO 2) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(-1 To 1) As T214

work(-1).nums(1) = -101
work(0).nums(2) = 2
work(1).nums(1) = 101
ok = (LBound(work) = -1 And UBound(work) = 1 And work(-1).nums(1) = -101 And work(0).nums(2) = 2 And work(1).nums(1) = 101)
If ok Then Print "PASS t214_parent_negative_bounds" Else Print "FAIL t214_parent_negative_bounds"
System
