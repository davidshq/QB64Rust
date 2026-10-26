$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T243
    nums(0 TO 5) _Dynamic As Long
End Type

Dim ok As _Byte
Dim idxA As Long
Dim idxB As Long
ReDim work(0) As T243

idxA = 2
idxB = idxA + 3
work(0).nums(idxA) = 200
work(0).nums(idxB) = 500
ok = (work(0).nums(2) = 200 And work(0).nums(5) = 500 And work(0).nums(idxA) + work(0).nums(idxB) = 700)
If ok Then Print "PASS t243_computed_index_access" Else Print "FAIL t243_computed_index_access"
System
