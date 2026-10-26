$Console:Only
$Unstable:TypeFields

Option _Explicit

Type Leaf274
    fixedNums(0 TO 1) _Static As Long
    dynNums(0 TO 1) _Dynamic As Long
End Type

Type T274
    fixedCodes(0 TO 1) _Static As String * 3
    items(0 TO 1) _Dynamic As Leaf274
End Type

Dim ok As _Byte
ReDim work(0 To 0) As T274

work(0).fixedCodes(0) = "A0"
work(0).fixedCodes(1) = "A1"
work(0).items(0).fixedNums(0) = 10
work(0).items(0).fixedNums(1) = 11
work(0).items(0).dynNums(0) = 100
work(0).items(0).dynNums(1) = 101
work(0).items(1).fixedNums(0) = 20
work(0).items(1).fixedNums(1) = 21

ReDim work(0).items(1).dynNums(3 TO 4)
work(0).items(1).dynNums(3) = 203
work(0).items(1).dynNums(4) = 204

ok = (work(0).fixedCodes(0) = "A0 " And work(0).fixedCodes(1) = "A1 ")
ok = ok And (LBound(work(0).items) = 0 And UBound(work(0).items) = 1)
ok = ok And (LBound(work(0).items(0).fixedNums) = 0 And UBound(work(0).items(0).fixedNums) = 1)
ok = ok And (work(0).items(0).fixedNums(0) = 10 And work(0).items(0).fixedNums(1) = 11)
ok = ok And (work(0).items(0).dynNums(0) = 100 And work(0).items(0).dynNums(1) = 101)
ok = ok And (work(0).items(1).fixedNums(0) = 20 And work(0).items(1).fixedNums(1) = 21)
ok = ok And (LBound(work(0).items(1).dynNums) = 3 And UBound(work(0).items(1).dynNums) = 4)
ok = ok And (work(0).items(1).dynNums(3) = 203 And work(0).items(1).dynNums(4) = 204)

If ok Then Print "PASS t274_nested_dynamic_static_owner" Else Print "FAIL t274_nested_dynamic_static_owner"
System
