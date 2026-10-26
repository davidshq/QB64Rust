$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T217
    numsA(0 TO 2) _Dynamic As Long
    numsB(10 TO 12) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T217

work(0).numsA(0) = 7
work(0).numsA(2) = 9
work(0).numsB(10) = 100
work(0).numsB(12) = 120
ok = (work(0).numsA(0) = 7 And work(0).numsA(2) = 9 And work(0).numsB(10) = 100 And work(0).numsB(12) = 120)
If ok Then Print "PASS t217_multi_members_independent" Else Print "FAIL t217_multi_members_independent"
System
