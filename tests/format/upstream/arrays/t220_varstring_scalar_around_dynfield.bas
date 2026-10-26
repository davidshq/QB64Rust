$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T220
    headText As String
    nums(0 TO 1) _Dynamic As Long
    tailText As String
End Type

Dim ok As _Byte
ReDim work(0) As T220

work(0).headText = "alpha"
work(0).nums(1) = 456
work(0).tailText = "omega"
ok = (work(0).headText = "alpha" And work(0).nums(1) = 456 And work(0).tailText = "omega")
If ok Then Print "PASS t220_varstring_scalar_around_dynfield" Else Print "FAIL t220_varstring_scalar_around_dynfield"
System
