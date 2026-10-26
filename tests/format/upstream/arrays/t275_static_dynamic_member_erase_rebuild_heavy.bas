$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T275
    fixedNums(0 TO 2) _Static As Long
    fixedCodes(0 TO 1) _Static As String * 4
    dynNums(0 TO 1) _Dynamic As Long
    dynCodes(0 TO 1) _Dynamic As String * 3
End Type

Dim ok As _Byte
ReDim work(0 To 1) As T275

work(0).fixedNums(0) = 10
work(0).fixedNums(1) = 11
work(0).fixedNums(2) = 12
work(0).fixedCodes(0) = "A0"
work(0).fixedCodes(1) = "A1"
work(0).dynNums(0) = 100
work(0).dynNums(1) = 101
work(0).dynCodes(0) = "X0"
work(0).dynCodes(1) = "X1"

work(1).fixedNums(0) = 20
work(1).fixedNums(1) = 21
work(1).fixedNums(2) = 22
work(1).fixedCodes(0) = "B0"
work(1).fixedCodes(1) = "B1"
ReDim work(1).dynNums(5 TO 7)
work(1).dynNums(5) = 205
work(1).dynNums(6) = 206
work(1).dynNums(7) = 207
ReDim work(1).dynCodes(3 TO 4)
work(1).dynCodes(3) = "Y3"
work(1).dynCodes(4) = "Y4"

Erase work(0).dynNums
Erase work(0).dynCodes
ReDim work(0).dynNums(-2 TO 0)
work(0).dynNums(-2) = -200
work(0).dynNums(-1) = -100
work(0).dynNums(0) = 0
ReDim work(0).dynCodes(8 TO 9)
work(0).dynCodes(8) = "N8"
work(0).dynCodes(9) = "N9"

ok = (work(0).fixedNums(0) = 10 And work(0).fixedNums(2) = 12)
ok = ok And (work(0).fixedCodes(0) = "A0  " And work(0).fixedCodes(1) = "A1  ")
ok = ok And (LBound(work(0).dynNums) = -2 And UBound(work(0).dynNums) = 0)
ok = ok And (work(0).dynNums(-2) = -200 And work(0).dynNums(-1) = -100 And work(0).dynNums(0) = 0)
ok = ok And (LBound(work(0).dynCodes) = 8 And UBound(work(0).dynCodes) = 9)
ok = ok And (work(0).dynCodes(8) = "N8 " And work(0).dynCodes(9) = "N9 ")
ok = ok And (work(1).fixedNums(0) = 20 And work(1).fixedNums(2) = 22)
ok = ok And (work(1).fixedCodes(0) = "B0  " And work(1).fixedCodes(1) = "B1  ")
ok = ok And (LBound(work(1).dynNums) = 5 And UBound(work(1).dynNums) = 7)
ok = ok And (work(1).dynNums(5) = 205 And work(1).dynNums(7) = 207)
ok = ok And (LBound(work(1).dynCodes) = 3 And UBound(work(1).dynCodes) = 4)
ok = ok And (work(1).dynCodes(3) = "Y3 " And work(1).dynCodes(4) = "Y4 ")

If ok Then Print "PASS t275_static_dynamic_member_erase_rebuild_heavy" Else Print "FAIL t275_static_dynamic_member_erase_rebuild_heavy"
System
