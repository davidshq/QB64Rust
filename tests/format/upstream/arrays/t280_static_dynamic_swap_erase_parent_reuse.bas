$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T280
    fixedNums(0 TO 1) _Static As Long
    dynNums(0 TO 1) _Dynamic As Long
    dynText(0 TO 1) _Dynamic As String
End Type

Dim ok As _Byte
ReDim work(0 To 2) As T280

work(0).fixedNums(0) = 10
work(0).fixedNums(1) = 11
ReDim work(0).dynNums(0 TO 2)
work(0).dynNums(0) = 100
work(0).dynNums(1) = 101
work(0).dynNums(2) = 102
work(0).dynText(0) = "zero-a"
work(0).dynText(1) = "zero-b"

work(2).fixedNums(0) = 30
work(2).fixedNums(1) = 31
ReDim work(2).dynNums(5 TO 6)
work(2).dynNums(5) = 305
work(2).dynNums(6) = 306
ReDim work(2).dynText(7 TO 8)
work(2).dynText(7) = "two-a"
work(2).dynText(8) = "two-b"

Swap work(0), work(2)

ok = (work(0).fixedNums(0) = 30 And work(0).fixedNums(1) = 31)
ok = ok And (LBound(work(0).dynNums) = 5 And UBound(work(0).dynNums) = 6)
ok = ok And (work(0).dynNums(5) = 305 And work(0).dynNums(6) = 306)
ok = ok And (LBound(work(0).dynText) = 7 And UBound(work(0).dynText) = 8)
ok = ok And (work(0).dynText(7) = "two-a" And work(0).dynText(8) = "two-b")
ok = ok And (work(2).fixedNums(0) = 10 And work(2).fixedNums(1) = 11)
ok = ok And (LBound(work(2).dynNums) = 0 And UBound(work(2).dynNums) = 2)
ok = ok And (work(2).dynNums(0) = 100 And work(2).dynNums(2) = 102)
ok = ok And (work(2).dynText(0) = "zero-a" And work(2).dynText(1) = "zero-b")

Erase work(2).dynNums
ReDim work(2).dynNums(-4 TO -3)
work(2).dynNums(-4) = -404
work(2).dynNums(-3) = -403
ok = ok And (work(2).fixedNums(0) = 10 And work(2).fixedNums(1) = 11)
ok = ok And (LBound(work(2).dynNums) = -4 And UBound(work(2).dynNums) = -3)
ok = ok And (work(2).dynNums(-4) = -404 And work(2).dynNums(-3) = -403)

Erase work
ReDim work(0 To 0) As T280
work(0).fixedNums(0) = 90
work(0).dynNums(0) = 900
work(0).dynText(0) = "fresh"
ok = ok And (LBound(work) = 0 And UBound(work) = 0)
ok = ok And (work(0).fixedNums(0) = 90 And work(0).fixedNums(1) = 0)
ok = ok And (LBound(work(0).dynNums) = 0 And UBound(work(0).dynNums) = 1)
ok = ok And (work(0).dynNums(0) = 900 And work(0).dynNums(1) = 0)
ok = ok And (work(0).dynText(0) = "fresh" And work(0).dynText(1) = "")

If ok Then Print "PASS t280_static_dynamic_swap_erase_parent_reuse" Else Print "FAIL t280_static_dynamic_swap_erase_parent_reuse"
System
