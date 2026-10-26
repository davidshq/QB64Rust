$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T234
    textItems(0 TO 2) As String
End Type

Dim ok As _Byte
ReDim work(0) As T234

work(0).textItems(0) = "zero"
work(0).textItems(1) = "one"
work(0).textItems(2) = "two"
ok = (work(0).textItems(0) = "zero" And work(0).textItems(1) = "one" And work(0).textItems(2) = "two")
If ok Then Print "PASS t234_unmarked_inline_varstring_array_redim_parent" Else Print "FAIL t234_unmarked_inline_varstring_array_redim_parent"
System
