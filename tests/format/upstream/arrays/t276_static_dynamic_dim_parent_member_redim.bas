$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T276
    fixedNums(0 TO 1) _Static As Long
    dynNums(0 TO 1) _Dynamic As Long
    dynNames(0 TO 1) _Dynamic As String
End Type

Dim ok As _Byte
Dim work(0 To 1) As T276

work(0).fixedNums(0) = 10
work(0).fixedNums(1) = 11
work(0).dynNums(0) = 100
work(0).dynNums(1) = 101
work(0).dynNames(0) = "alpha"
work(0).dynNames(1) = "beta"

work(1).fixedNums(0) = 20
work(1).fixedNums(1) = 21
ReDim work(1).dynNums(4 TO 5)
work(1).dynNums(4) = 204
work(1).dynNums(5) = 205
ReDim work(1).dynNames(2 TO 4)
work(1).dynNames(2) = "two"
work(1).dynNames(3) = "three"
work(1).dynNames(4) = "four"

ReDim _Retain work(0).dynNums(0 TO 3)
work(0).dynNums(2) = 102
work(0).dynNums(3) = 103
Erase work(1).dynNames
ReDim work(1).dynNames(-1 TO 0)
work(1).dynNames(-1) = "minus"
work(1).dynNames(0) = "zero"

ok = (LBound(work) = 0 And UBound(work) = 1)
ok = ok And (work(0).fixedNums(0) = 10 And work(0).fixedNums(1) = 11)
ok = ok And (LBound(work(0).dynNums) = 0 And UBound(work(0).dynNums) = 3)
ok = ok And (work(0).dynNums(0) = 100 And work(0).dynNums(1) = 101 And work(0).dynNums(2) = 102 And work(0).dynNums(3) = 103)
ok = ok And (work(0).dynNames(0) = "alpha" And work(0).dynNames(1) = "beta")
ok = ok And (work(1).fixedNums(0) = 20 And work(1).fixedNums(1) = 21)
ok = ok And (LBound(work(1).dynNums) = 4 And UBound(work(1).dynNums) = 5)
ok = ok And (work(1).dynNums(4) = 204 And work(1).dynNums(5) = 205)
ok = ok And (LBound(work(1).dynNames) = -1 And UBound(work(1).dynNames) = 0)
ok = ok And (work(1).dynNames(-1) = "minus" And work(1).dynNames(0) = "zero")

If ok Then Print "PASS t276_static_dynamic_dim_parent_member_redim" Else Print "FAIL t276_static_dynamic_dim_parent_member_redim"
System
