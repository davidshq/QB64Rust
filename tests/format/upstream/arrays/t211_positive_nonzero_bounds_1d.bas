$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T211
    nums(5 TO 7) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T211

work(0).nums(5) = 105
work(0).nums(6) = 106
work(0).nums(7) = 107

ok = (LBound(work(0).nums) = 5 And UBound(work(0).nums) = 7 And work(0).nums(5) + work(0).nums(7) = 212)
If ok Then Print "PASS t211_positive_nonzero_bounds_1d" Else Print "FAIL t211_positive_nonzero_bounds_1d"
System
