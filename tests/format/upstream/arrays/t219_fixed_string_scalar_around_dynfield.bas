$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T219
    headText As String * 4
    nums(0 TO 1) _Dynamic As Long
    tailText As String * 4
End Type

Dim ok As _Byte
ReDim work(0) As T219

work(0).headText = "HEAD"
work(0).nums(0) = 123
work(0).tailText = "TAIL"
ok = (work(0).headText = "HEAD" And work(0).nums(0) = 123 And work(0).tailText = "TAIL")
If ok Then Print "PASS t219_fixed_string_scalar_around_dynfield" Else Print "FAIL t219_fixed_string_scalar_around_dynfield"
System
