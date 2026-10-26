$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T215
    grid(2 TO 3, -1 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T215

work(0).grid(2, -1) = 21
work(0).grid(3, 1) = 31
ok = (LBound(work(0).grid, 1) = 2 And UBound(work(0).grid, 1) = 3 And LBound(work(0).grid, 2) = -1 And UBound(work(0).grid, 2) = 1 And work(0).grid(2, -1) = 21 And work(0).grid(3, 1) = 31)
If ok Then Print "PASS t215_two_dim_mixed_bounds" Else Print "FAIL t215_two_dim_mixed_bounds"
System
