$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T279
    fixedGrid(0 TO 1, 0 TO 1) _Static As Long
    dynGrid(0 TO 1, 0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0 To 0) As T279

work(0).fixedGrid(0, 0) = 11
work(0).fixedGrid(0, 1) = 12
work(0).fixedGrid(1, 0) = 21
work(0).fixedGrid(1, 1) = 22
work(0).dynGrid(0, 0) = 100
work(0).dynGrid(0, 1) = 101
work(0).dynGrid(1, 0) = 110
work(0).dynGrid(1, 1) = 111

ReDim _Retain work(0 To 1) As T279
work(1).fixedGrid(0, 0) = 31
work(1).fixedGrid(1, 1) = 44
work(1).dynGrid(0, 0) = 300
work(1).dynGrid(1, 1) = 311

ReDim _Retain work(0).dynGrid(0 TO 2, 0 TO 2)
work(0).dynGrid(2, 0) = 120
work(0).dynGrid(2, 2) = 122

ok = (LBound(work) = 0 And UBound(work) = 1)
ok = ok And (work(0).fixedGrid(0, 0) = 11 And work(0).fixedGrid(1, 1) = 22)
ok = ok And (LBound(work(0).dynGrid, 1) = 0 And UBound(work(0).dynGrid, 1) = 2)
ok = ok And (LBound(work(0).dynGrid, 2) = 0 And UBound(work(0).dynGrid, 2) = 2)
ok = ok And (work(0).dynGrid(0, 0) = 100 And work(0).dynGrid(1, 1) = 111)
ok = ok And (work(0).dynGrid(2, 0) = 120 And work(0).dynGrid(2, 2) = 122)
ok = ok And (work(1).fixedGrid(0, 0) = 31 And work(1).fixedGrid(1, 1) = 44)
ok = ok And (LBound(work(1).dynGrid, 1) = 0 And UBound(work(1).dynGrid, 1) = 1)
ok = ok And (LBound(work(1).dynGrid, 2) = 0 And UBound(work(1).dynGrid, 2) = 1)
ok = ok And (work(1).dynGrid(0, 0) = 300 And work(1).dynGrid(1, 1) = 311)

If ok Then Print "PASS t279_static_dynamic_2d_retain_parent_resize" Else Print "FAIL t279_static_dynamic_2d_retain_parent_resize"
System
