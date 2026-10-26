$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T267
    labelText As String * 8
    nums(1 TO 2) _Dynamic As Integer
End Type

Dim ok As _Byte
Dim fileName As String
Dim fileNum As Integer
ReDim src(1 To 2) As T267
ReDim dst(1 To 2) As T267

fileName = "t267_putget_fixed_scalar_string_nonzero_bounds.bin"
If _FileExists(fileName) Then Kill fileName

src(1).labelText = "FIRST001"
src(1).nums(1) = 111
src(1).nums(2) = 112

src(2).labelText = "SECOND02"
src(2).nums(1) = 221
src(2).nums(2) = 222

fileNum = FreeFile
Open fileName For Binary As #fileNum
Put #fileNum, , src()
Close #fileNum

fileNum = FreeFile
Open fileName For Binary As #fileNum
Get #fileNum, , dst()
Close #fileNum

ok = (LBound(dst) = 1 And UBound(dst) = 2)
ok = ok And (LBound(dst(1).nums) = 1 And UBound(dst(1).nums) = 2)
ok = ok And (dst(1).labelText = "FIRST001" And dst(1).nums(1) = 111 And dst(1).nums(2) = 112)
ok = ok And (dst(2).labelText = "SECOND02" And dst(2).nums(1) = 221 And dst(2).nums(2) = 222)

If _FileExists(fileName) Then Kill fileName
If ok Then Print "PASS t267_putget_fixed_scalar_string_nonzero_bounds" Else Print "FAIL t267_putget_fixed_scalar_string_nonzero_bounds"
System
