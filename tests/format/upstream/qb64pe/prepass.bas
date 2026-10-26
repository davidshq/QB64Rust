DefLng A-Z
$Console:Only

Dim Debug As Long

'$Include:'../../../source/global/constants.bas'
sp = "@" ' Makes the output readable

'$Include:'../../../source/utilities/const_eval.bi'
'$Include:'../../../source/utilities/ini-manager/ini.bi'
'$Include:'../../../source/utilities/s-buffer/simplebuffer.bi'
'$Include:'../../../source/utilities/hash.bi'
'$Include:'../../../source/utilities/type.bi'
'$Include:'../../../source/utilities/give_error.bi'

Dim tests(4) As String
' These tests cover the paren insert around NOT, and some simple cases
tests(1) = "(@20@+@40@+@(@60@*@4@AND@50@+@NOT@5@+@4@)@-@2@)"
tests(2) = "(@20@+@40%@+@60000000&&@+@_RGB32@(@20@,@50@,@60@)@+@(@60@*@4@AND@50@+@NOT@5@+@4@)@-@2@)"
tests(3) = "2@+@NOT@5@+@2@*@6@^@3"
tests(4) = "2@+@-@2@"

For i = 1 To UBound(tests)
    Print "Test: "; Readable$(tests(i))
    PreParse tests(i)
    Print "PrePass: "; Readable$(tests(i))
Next i

' Test empty string
test2$ = ""
Print "Test: "; Readable$(test2$)
PreParse test2$
Print "PrePass: "; Readable$(test2$)

Dim errs(5) As String

' Various invalid paren cases
errs(1) = ")@("
errs(2) = "(@(@)@)@)"
errs(3) = "(@(@(@)@)"
errs(4) = "("
errs(5) = ")"

For i = 1 To UBound(errs)
    Print "Test: "; Readable$(errs(i))
    PreParse errs(i)
    Print "PrePass: "; Readable$(errs(i))
Next

System

qberror_test:
E = 1
Resume Next

'$Include:'../../../source/utilities/ini-manager/ini.bm'
'$Include:'../../../source/utilities/s-buffer/simplebuffer.bm'
'$Include:'../../../source/utilities/elements.bas'
'$Include:'../../../source/utilities/const_eval.bas'
'$Include:'../../../source/utilities/hash.bas'
'$Include:'../../../source/utilities/file.bas'
'$Include:'../../../source/utilities/give_error.bas'
'$Include:'../../../source/utilities/strings.bas'
'$Include:'../../../source/utilities/type.bas'

Function Readable$ (a$)
    r$ = ""

    For i = 1 To numelements(a$)
        r$ = r$ + getelement$(a$, i) + " "
    Next

    Readable$ = r$
End Function
