$Console:Only
$Unstable:TypeFields

Option _Explicit

Type Leaf246
    numsA(0 TO 1) _Dynamic As Long
    gapText As String
    numsB(0 TO 1) _Dynamic As Long
End Type

Type T246
    item(0 TO 1) _Dynamic As Leaf246
End Type

Dim ok As _Byte
ReDim work(0) As T246

work(0).item(0).numsA(1) = 12
work(0).item(0).gapText = "gap"
work(0).item(0).numsB(0) = 34
work(0).item(1).gapText = "second"
work(0).item(1).numsB(1) = 56
ok = (work(0).item(0).numsA(1) = 12 And work(0).item(0).gapText = "gap" And work(0).item(0).numsB(0) = 34 And work(0).item(1).gapText = "second" And work(0).item(1).numsB(1) = 56)
If ok Then Print "PASS t246_nested_owner_scalar_gap" Else Print "FAIL t246_nested_owner_scalar_gap"
System
