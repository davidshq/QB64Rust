$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T249
    codes(1 TO 2, 1 TO 2) _Dynamic As String * 3
End Type

Dim ok As _Byte
ReDim work(0) As T249

work(0).codes(1, 1) = "A"
work(0).codes(1, 2) = "BC"
work(0).codes(2, 1) = "DEF"
work(0).codes(2, 2) = "G"
ok = (RTrim$(work(0).codes(1, 1)) = "A" And RTrim$(work(0).codes(1, 2)) = "BC" And work(0).codes(2, 1) = "DEF" And RTrim$(work(0).codes(2, 2)) = "G")
If ok Then Print "PASS t249_fixed_len_string_2d_dynamic_field" Else Print "FAIL t249_fixed_len_string_2d_dynamic_field"
System
