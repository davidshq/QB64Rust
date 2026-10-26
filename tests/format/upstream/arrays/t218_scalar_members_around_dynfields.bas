$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T218
    idNum As Long
    numsA(0 TO 1) _Dynamic As Long
    middleNum As Long
    numsB(0 TO 1) _Dynamic As Long
    tailNum As Long
End Type

Dim ok As _Byte
ReDim work(0) As T218

work(0).idNum = 11
work(0).numsA(1) = 22
work(0).middleNum = 33
work(0).numsB(0) = 44
work(0).tailNum = 55
ok = (work(0).idNum = 11 And work(0).numsA(1) = 22 And work(0).middleNum = 33 And work(0).numsB(0) = 44 And work(0).tailNum = 55)
If ok Then Print "PASS t218_scalar_members_around_dynfields" Else Print "FAIL t218_scalar_members_around_dynfields"
System
