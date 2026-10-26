$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T265
    nums(0 TO 1) _Dynamic As Long
    tagText As String
End Type

Dim ok As _Byte
ReDim work(0 To 1) As T265

ReDim work(0).nums(0 TO 2)
work(0).nums(0) = 10
work(0).nums(1) = 11
work(0).nums(2) = 12
work(0).tagText = "zero"

ReDim work(1).nums(5 TO 6)
work(1).nums(5) = 50
work(1).nums(6) = 60
work(1).tagText = "one"

Swap work(0), work(1)

ok = (LBound(work(0).nums) = 5 And UBound(work(0).nums) = 6)
ok = ok And (work(0).nums(5) = 50 And work(0).nums(6) = 60 And work(0).tagText = "one")
ok = ok And (LBound(work(1).nums) = 0 And UBound(work(1).nums) = 2)
ok = ok And (work(1).nums(0) = 10 And work(1).nums(1) = 11 And work(1).nums(2) = 12 And work(1).tagText = "zero")

If ok Then Print "PASS t265_swap_parent_elements_different_member_bounds" Else Print "FAIL t265_swap_parent_elements_different_member_bounds"
System
