$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T266
    codes(0 TO 1) _Dynamic As String * 4
    nums(0 TO 2) _Dynamic As Long
End Type

Dim ok As _Byte
Dim fileName As String
Dim fileNum As Integer
ReDim src(0 To 1) As T266
ReDim dst(0 To 1) As T266

fileName = "t266_putget_fixed_dynamic_fields.bin"
If _FileExists(fileName) Then Kill fileName

src(0).codes(0) = "A000"
src(0).codes(1) = "A111"
src(0).nums(0) = 10
src(0).nums(1) = 11
src(0).nums(2) = 12

src(1).codes(0) = "B000"
src(1).codes(1) = "B111"
src(1).nums(0) = 20
src(1).nums(1) = 21
src(1).nums(2) = 22

fileNum = FreeFile
Open fileName For Binary As #fileNum
Put #fileNum, , src()
Close #fileNum

fileNum = FreeFile
Open fileName For Binary As #fileNum
Get #fileNum, , dst()
Close #fileNum

ok = (dst(0).codes(0) = "A000" And dst(0).codes(1) = "A111")
ok = ok And (dst(0).nums(0) = 10 And dst(0).nums(1) = 11 And dst(0).nums(2) = 12)
ok = ok And (dst(1).codes(0) = "B000" And dst(1).codes(1) = "B111")
ok = ok And (dst(1).nums(0) = 20 And dst(1).nums(1) = 21 And dst(1).nums(2) = 22)

If _FileExists(fileName) Then Kill fileName
If ok Then Print "PASS t266_putget_fixed_dynamic_fields" Else Print "FAIL t266_putget_fixed_dynamic_fields"
System
