$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T277
    fixedNums(0 TO 2) _Static As Long
    dynNums(0 TO 1) _Dynamic As Long
    dynText(0 TO 1) _Dynamic As String * 4
End Type

Dim ok As _Byte
ReDim work(1 To 1) As T277

work(1).fixedNums(0) = 10
work(1).fixedNums(1) = 11
work(1).fixedNums(2) = 12
ReDim work(1).dynNums(3 TO 4)
work(1).dynNums(3) = 103
work(1).dynNums(4) = 104
work(1).dynText(0) = "A000"
work(1).dynText(1) = "A111"

ReDim _Retain work(1 To 2) As T277
work(2).fixedNums(0) = 20
work(2).fixedNums(1) = 21
work(2).fixedNums(2) = 22
ReDim work(2).dynNums(-2 TO -1)
work(2).dynNums(-2) = -202
work(2).dynNums(-1) = -201
work(2).dynText(0) = "B000"
work(2).dynText(1) = "B111"

ok = (LBound(work) = 1 And UBound(work) = 2)
ok = ok And (work(1).fixedNums(0) = 10 And work(1).fixedNums(2) = 12)
ok = ok And (LBound(work(1).dynNums) = 3 And UBound(work(1).dynNums) = 4)
ok = ok And (work(1).dynNums(3) = 103 And work(1).dynNums(4) = 104)
ok = ok And (work(1).dynText(0) = "A000" And work(1).dynText(1) = "A111")
ok = ok And (work(2).fixedNums(0) = 20 And work(2).fixedNums(2) = 22)
ok = ok And (LBound(work(2).dynNums) = -2 And UBound(work(2).dynNums) = -1)
ok = ok And (work(2).dynNums(-2) = -202 And work(2).dynNums(-1) = -201)

ReDim work(1 To 1) As T277
ok = ok And (LBound(work) = 1 And UBound(work) = 1)
ok = ok And (work(1).fixedNums(0) = 0 And work(1).fixedNums(1) = 0 And work(1).fixedNums(2) = 0)
ok = ok And (LBound(work(1).dynNums) = 0 And UBound(work(1).dynNums) = 1)
ok = ok And (work(1).dynNums(0) = 0 And work(1).dynNums(1) = 0)
ok = ok And (LBound(work(1).dynText) = 0 And UBound(work(1).dynText) = 1)
ok = ok And (work(1).dynText(0) = String$(4, 0) And work(1).dynText(1) = String$(4, 0))

If ok Then Print "PASS t277_static_dynamic_parent_redim_reset_retain" Else Print "FAIL t277_static_dynamic_parent_redim_reset_retain"
System
