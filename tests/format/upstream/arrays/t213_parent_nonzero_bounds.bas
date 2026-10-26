$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T213
    nums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(3 To 4) As T213

work(3).nums(0) = 30
work(4).nums(1) = 41
ok = (LBound(work) = 3 And UBound(work) = 4 And work(3).nums(0) = 30 And work(4).nums(1) = 41)
If ok Then Print "PASS t213_parent_nonzero_bounds" Else Print "FAIL t213_parent_nonzero_bounds"
System
