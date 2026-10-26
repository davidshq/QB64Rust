$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T232
    codes(0 TO 1) _Static As String * 4
    nums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T232

work(0).codes(0) = "ABCD"
work(0).codes(1) = "EF"
work(0).nums(0) = 12
work(0).nums(1) = 34
ok = (work(0).codes(0) = "ABCD" And RTrim$(work(0).codes(1)) = "EF" And work(0).nums(0) + work(0).nums(1) = 46)
If ok Then Print "PASS t232_staticfield_fixed_len_string_array" Else Print "FAIL t232_staticfield_fixed_len_string_array"
System
