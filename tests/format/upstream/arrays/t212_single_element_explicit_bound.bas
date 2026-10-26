$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T212
    nums(4 TO 4) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T212

work(0).nums(4) = 444
ok = (LBound(work(0).nums) = 4 And UBound(work(0).nums) = 4 And work(0).nums(4) = 444)
If ok Then Print "PASS t212_single_element_explicit_bound" Else Print "FAIL t212_single_element_explicit_bound"
System
