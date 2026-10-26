$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T231
    codes(1 TO 3) _Dynamic As String * 5
End Type

Dim ok As _Byte
ReDim work(0) As T231

work(0).codes(1) = "ALPHA"
work(0).codes(2) = "B"
work(0).codes(3) = "GAMMA"
ok = (work(0).codes(1) = "ALPHA" And RTrim$(work(0).codes(2)) = "B" And work(0).codes(3) = "GAMMA")
If ok Then Print "PASS t231_fixed_len_string_dynamic_field" Else Print "FAIL t231_fixed_len_string_dynamic_field"
System
