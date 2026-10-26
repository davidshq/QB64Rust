$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T240
    nums(0 TO 2) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T240

work(0).nums(2) = 222
Erase work(0).nums
ReDim work(0).nums(1 TO 2)
work(0).nums(1) = 111
work(0).nums(2) = 222
ok = (LBound(work(0).nums) = 1 And UBound(work(0).nums) = 2 And work(0).nums(1) = 111 And work(0).nums(2) = 222)
If ok Then Print "PASS t240_erase_member_then_redim" Else Print "FAIL t240_erase_member_then_redim"
System
