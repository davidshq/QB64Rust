$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T271
    codes(0 TO 1) _Static As String * 4
    nums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0 To 0) As T271

work(0).codes(0) = "A000"
work(0).codes(1) = "A111"
work(0).nums(0) = 10
work(0).nums(1) = 11

ReDim _Retain work(0 To 1) As T271
work(1).codes(0) = "B000"
work(1).codes(1) = "B111"
work(1).nums(0) = 20
work(1).nums(1) = 21

ok = (LBound(work) = 0 And UBound(work) = 1)
ok = ok And (work(0).codes(0) = "A000" And work(0).codes(1) = "A111")
ok = ok And (work(0).nums(0) = 10 And work(0).nums(1) = 11)
ok = ok And (work(1).codes(0) = "B000" And work(1).codes(1) = "B111")
ok = ok And (work(1).nums(0) = 20 And work(1).nums(1) = 21)

ReDim _Retain work(0).nums(0 TO 3)
work(0).nums(2) = 12
work(0).nums(3) = 13

ok = ok And (LBound(work(0).nums) = 0 And UBound(work(0).nums) = 3)
ok = ok And (work(0).nums(0) = 10 And work(0).nums(1) = 11 And work(0).nums(2) = 12 And work(0).nums(3) = 13)
ok = ok And (work(0).codes(0) = "A000" And work(0).codes(1) = "A111")

If ok Then Print "PASS t271_dynamic_static_parent_retain" Else Print "FAIL t271_dynamic_static_parent_retain"
System
