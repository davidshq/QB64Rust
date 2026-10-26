$Console:Only
Option _Explicit
$Unstable:TypeFields

Type Leaf278
    fixedNums(0 TO 1) _Static As Long
    fixedTag As String * 4
    dynNums(0 TO 1) _Dynamic As Long
End Type

Type T278
    fixedSlots(0 TO 1) _Static As Long
    leaves(0 TO 1) _Dynamic As Leaf278
End Type

Dim ok As _Byte
ReDim src(0 To 0) As T278
ReDim dst(0 To 0) As T278

src(0).fixedSlots(0) = 7
src(0).fixedSlots(1) = 8
src(0).leaves(0).fixedNums(0) = 10
src(0).leaves(0).fixedNums(1) = 11
src(0).leaves(0).fixedTag = "L0"
src(0).leaves(0).dynNums(0) = 100
src(0).leaves(0).dynNums(1) = 101
src(0).leaves(1).fixedNums(0) = 20
src(0).leaves(1).fixedNums(1) = 21
src(0).leaves(1).fixedTag = "L1"
ReDim src(0).leaves(1).dynNums(5 TO 6)
src(0).leaves(1).dynNums(5) = 205
src(0).leaves(1).dynNums(6) = 206

dst() = src()

src(0).fixedSlots(0) = -7
src(0).leaves(0).fixedNums(0) = -10
src(0).leaves(0).dynNums(0) = -100
Erase src(0).leaves(1).dynNums
ReDim src(0).leaves(1).dynNums(-3 TO -2)
src(0).leaves(1).dynNums(-3) = -303
src(0).leaves(1).dynNums(-2) = -302

ok = (dst(0).fixedSlots(0) = 7 And dst(0).fixedSlots(1) = 8)
ok = ok And (dst(0).leaves(0).fixedNums(0) = 10 And dst(0).leaves(0).fixedNums(1) = 11)
ok = ok And (dst(0).leaves(0).fixedTag = "L0  ")
ok = ok And (LBound(dst(0).leaves(0).dynNums) = 0 And UBound(dst(0).leaves(0).dynNums) = 1)
ok = ok And (dst(0).leaves(0).dynNums(0) = 100 And dst(0).leaves(0).dynNums(1) = 101)
ok = ok And (dst(0).leaves(1).fixedNums(0) = 20 And dst(0).leaves(1).fixedNums(1) = 21)
ok = ok And (dst(0).leaves(1).fixedTag = "L1  ")
ok = ok And (LBound(dst(0).leaves(1).dynNums) = 5 And UBound(dst(0).leaves(1).dynNums) = 6)
ok = ok And (dst(0).leaves(1).dynNums(5) = 205 And dst(0).leaves(1).dynNums(6) = 206)
ok = ok And (src(0).leaves(1).fixedNums(0) = 20 And src(0).leaves(1).fixedNums(1) = 21)
ok = ok And (LBound(src(0).leaves(1).dynNums) = -3 And UBound(src(0).leaves(1).dynNums) = -2)
ok = ok And (src(0).leaves(1).dynNums(-3) = -303 And src(0).leaves(1).dynNums(-2) = -302)

If ok Then Print "PASS t278_nested_static_dynamic_copy_erase_rebuild" Else Print "FAIL t278_nested_static_dynamic_copy_erase_rebuild"
System
