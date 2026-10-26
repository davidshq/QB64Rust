$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T264
    nums(0 TO 2) _Dynamic As Long
    marker As Long
End Type

Dim ok As _Byte
ReDim work(0) As T264

work(0).nums(0) = 7
work(0).nums(1) = 8
work(0).nums(2) = 9
work(0).marker = 1234

Erase work(0).nums
ReDim work(0).nums(2 TO 4)

ok = (LBound(work(0).nums) = 2 And UBound(work(0).nums) = 4)
ok = ok And (work(0).nums(2) = 0 And work(0).nums(3) = 0 And work(0).nums(4) = 0)
ok = ok And (work(0).marker = 1234)

work(0).nums(2) = 20
work(0).nums(4) = 40
ok = ok And (work(0).nums(2) = 20 And work(0).nums(4) = 40)

If ok Then Print "PASS t264_erase_member_then_rebuild_bounds" Else Print "FAIL t264_erase_member_then_rebuild_bounds"
System
