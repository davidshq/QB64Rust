$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T220
    headText As String
    nums(0 TO 1) _Dynamic As Long
    tailText As String
End Type

Dim ok As _Byte
ReDim work As T220

work.headText = "alpha"
work.nums(1) = 456
work.tailText = "omega"
ok = (work.headText = "alpha" And work.nums(1) = 456 And work.tailText = "omega")
If ok Then Print "PASS t220_2" Else Print "FAIL t220_2"
System

