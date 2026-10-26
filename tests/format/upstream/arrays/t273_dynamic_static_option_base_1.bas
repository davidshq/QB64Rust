$Console:Only
$Unstable:TypeFields

Option _Explicit
Option Base 1

Type T273
    fixedNums(3) _Static As Long
    dynText(2) _Dynamic As String * 4
End Type

Dim ok As _Byte
ReDim work(1 To 1) As T273

work(1).fixedNums(1) = 11
work(1).fixedNums(2) = 12
work(1).fixedNums(3) = 13
work(1).dynText(1) = "A001"
work(1).dynText(2) = "A002"

ok = (LBound(work(1).fixedNums) = 1 And UBound(work(1).fixedNums) = 3)
ok = ok And (LBound(work(1).dynText) = 1 And UBound(work(1).dynText) = 2)
ok = ok And (work(1).fixedNums(1) = 11 And work(1).fixedNums(3) = 13)
ok = ok And (work(1).dynText(1) = "A001" And work(1).dynText(2) = "A002")

ReDim _Retain work(1).dynText(1 TO 3)
work(1).dynText(3) = "A003"

ok = ok And (LBound(work(1).dynText) = 1 And UBound(work(1).dynText) = 3)
ok = ok And (work(1).dynText(1) = "A001" And work(1).dynText(2) = "A002" And work(1).dynText(3) = "A003")
ok = ok And (work(1).fixedNums(1) = 11 And work(1).fixedNums(2) = 12 And work(1).fixedNums(3) = 13)

If ok Then Print "PASS t273_dynamic_static_option_base_1" Else Print "FAIL t273_dynamic_static_option_base_1"
System
