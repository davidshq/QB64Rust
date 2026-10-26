$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T270
    fixedNums(0 TO 2) _Static As Long
    dynNums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0 To 1) As T270

work(0).fixedNums(0) = 10
work(0).fixedNums(1) = 11
work(0).fixedNums(2) = 12
work(1).fixedNums(0) = 20
work(1).fixedNums(1) = 21
work(1).fixedNums(2) = 22

ReDim work(0).dynNums(0 TO 2)
work(0).dynNums(0) = 100
work(0).dynNums(1) = 101
work(0).dynNums(2) = 102

ReDim work(1).dynNums(5 TO 6)
work(1).dynNums(5) = 205
work(1).dynNums(6) = 206

ok = (LBound(work(0).fixedNums) = 0 And UBound(work(0).fixedNums) = 2)
ok = ok And (LBound(work(1).fixedNums) = 0 And UBound(work(1).fixedNums) = 2)
ok = ok And (work(0).fixedNums(0) = 10 And work(0).fixedNums(2) = 12)
ok = ok And (work(1).fixedNums(0) = 20 And work(1).fixedNums(2) = 22)
ok = ok And (LBound(work(0).dynNums) = 0 And UBound(work(0).dynNums) = 2)
ok = ok And (LBound(work(1).dynNums) = 5 And UBound(work(1).dynNums) = 6)
ok = ok And (work(0).dynNums(0) = 100 And work(0).dynNums(2) = 102)
ok = ok And (work(1).dynNums(5) = 205 And work(1).dynNums(6) = 206)

If ok Then Print "PASS t270_dynamic_static_same_type_bounds" Else Print "FAIL t270_dynamic_static_same_type_bounds"
System
