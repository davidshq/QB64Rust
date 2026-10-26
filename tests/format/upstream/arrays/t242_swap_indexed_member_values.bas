$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T242
    nums(0 TO 2) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T242

work(0).nums(0) = 5
work(0).nums(2) = 9
Swap work(0).nums(0), work(0).nums(2)
ok = (work(0).nums(0) = 9 And work(0).nums(2) = 5)
If ok Then Print "PASS t242_swap_indexed_member_values" Else Print "FAIL t242_swap_indexed_member_values"
System
