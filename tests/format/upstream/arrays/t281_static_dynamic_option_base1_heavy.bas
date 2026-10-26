$Console:Only
$Unstable:TypeFields

Option _Explicit
Option Base 1

Type T281
    fixedNums(3) _Static As Long
    dynNums(2) _Dynamic As Long
    dynCodes(2) _Dynamic As String * 3
End Type

Dim ok As _Byte
ReDim work(1 To 2) As T281

work(1).fixedNums(1) = 11
work(1).fixedNums(2) = 12
work(1).fixedNums(3) = 13
work(1).dynNums(1) = 101
work(1).dynNums(2) = 102
work(1).dynCodes(1) = "A1"
work(1).dynCodes(2) = "A2"

ReDim work(2).dynNums(4 TO 5)
work(2).fixedNums(1) = 21
work(2).fixedNums(2) = 22
work(2).fixedNums(3) = 23
work(2).dynNums(4) = 204
work(2).dynNums(5) = 205
ReDim work(2).dynCodes(6 TO 7)
work(2).dynCodes(6) = "B6"
work(2).dynCodes(7) = "B7"

ReDim _Retain work(1 To 3) As T281
work(3).fixedNums(1) = 31
work(3).fixedNums(3) = 33
work(3).dynNums(1) = 301
work(3).dynCodes(1) = "C1"

Erase work(1).dynNums
ReDim work(1).dynNums(-1 TO 1)
work(1).dynNums(-1) = -101
work(1).dynNums(0) = 0
work(1).dynNums(1) = 101

ok = (LBound(work) = 1 And UBound(work) = 3)
ok = ok And (LBound(work(1).fixedNums) = 1 And UBound(work(1).fixedNums) = 3)
ok = ok And (work(1).fixedNums(1) = 11 And work(1).fixedNums(3) = 13)
ok = ok And (LBound(work(1).dynNums) = -1 And UBound(work(1).dynNums) = 1)
ok = ok And (work(1).dynNums(-1) = -101 And work(1).dynNums(0) = 0 And work(1).dynNums(1) = 101)
ok = ok And (LBound(work(1).dynCodes) = 1 And UBound(work(1).dynCodes) = 2)
ok = ok And (work(1).dynCodes(1) = "A1 " And work(1).dynCodes(2) = "A2 ")
ok = ok And (work(2).fixedNums(1) = 21 And work(2).fixedNums(3) = 23)
ok = ok And (LBound(work(2).dynNums) = 4 And UBound(work(2).dynNums) = 5)
ok = ok And (work(2).dynNums(4) = 204 And work(2).dynNums(5) = 205)
ok = ok And (LBound(work(2).dynCodes) = 6 And UBound(work(2).dynCodes) = 7)
ok = ok And (work(2).dynCodes(6) = "B6 " And work(2).dynCodes(7) = "B7 ")
ok = ok And (work(3).fixedNums(1) = 31 And work(3).fixedNums(2) = 0 And work(3).fixedNums(3) = 33)
ok = ok And (LBound(work(3).dynNums) = 1 And UBound(work(3).dynNums) = 2)
ok = ok And (work(3).dynNums(1) = 301 And work(3).dynNums(2) = 0)
ok = ok And (work(3).dynCodes(1) = "C1 " And work(3).dynCodes(2) = String$(3, 0))

If ok Then Print "PASS t281_static_dynamic_option_base1_heavy" Else Print "FAIL t281_static_dynamic_option_base1_heavy"
System
