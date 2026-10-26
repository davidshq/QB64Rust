$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T260
    nums(1 TO 3) _Dynamic As Long
    tagText As String * 4
End Type

Dim ok As _Byte
ReDim work(0 To 1) As T260

work(0).nums(1) = 10
work(0).nums(2) = 20
work(0).nums(3) = 30
work(0).tagText = "ZERO"

work(1).nums(1) = 101
work(1).nums(2) = 102
work(1).nums(3) = 103
work(1).tagText = "ONE1"

ReDim _Preserve work(1).nums(1 TO 5)
work(1).nums(4) = 104
work(1).nums(5) = 105

ok = (LBound(work(1).nums) = 1 And UBound(work(1).nums) = 5)
ok = ok And (work(1).nums(1) = 101 And work(1).nums(2) = 102 And work(1).nums(3) = 103)
ok = ok And (work(1).nums(4) = 104 And work(1).nums(5) = 105)
ok = ok And (work(0).nums(1) = 10 And work(0).nums(3) = 30 And work(0).tagText = "ZERO")

If ok Then Print "PASS t260_preserve_1d_member_expand_nonzero_parent" Else Print "FAIL t260_preserve_1d_member_expand_nonzero_parent"
System
