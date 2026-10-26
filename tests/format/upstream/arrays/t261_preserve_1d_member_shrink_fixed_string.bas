$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T261
    names(0 TO 4) _Dynamic As String * 6
End Type

Dim ok As _Byte
ReDim work(0) As T261

work(0).names(0) = "AA0000"
work(0).names(1) = "BB1111"
work(0).names(2) = "CC2222"
work(0).names(3) = "DD3333"
work(0).names(4) = "EE4444"

ReDim _Preserve work(0).names(0 TO 2)

ok = (LBound(work(0).names) = 0 And UBound(work(0).names) = 2)
ok = ok And (work(0).names(0) = "AA0000" And work(0).names(1) = "BB1111" And work(0).names(2) = "CC2222")

If ok Then Print "PASS t261_preserve_1d_member_shrink_fixed_string" Else Print "FAIL t261_preserve_1d_member_shrink_fixed_string"
System
