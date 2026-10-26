$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T238
    nums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T238

work(0).nums(0) = 101
work(0).nums(1) = 202
ReDim _Preserve work(0).nums(0 TO 2)
work(0).nums(2) = 303
ok = (LBound(work(0).nums) = 0 And UBound(work(0).nums) = 2 And work(0).nums(0) = 101 And work(0).nums(1) = 202 And work(0).nums(2) = 303)
If ok Then Print "PASS t238_member_redim_preserve_expand" Else Print "FAIL t238_member_redim_preserve_expand"
System
