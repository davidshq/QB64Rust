$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T216
    cube(0 TO 1, 0 TO 1, 0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T216

work(0).cube(0, 0, 0) = 1
work(0).cube(1, 1, 1) = 8
ok = (LBound(work(0).cube, 3) = 0 And UBound(work(0).cube, 3) = 1 And work(0).cube(0, 0, 0) = 1 And work(0).cube(1, 1, 1) = 8)
If ok Then Print "PASS t216_three_dim_small_cube" Else Print "FAIL t216_three_dim_small_cube"
System
