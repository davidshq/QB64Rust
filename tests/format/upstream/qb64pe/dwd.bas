DefLng A-Z
$Console:Only

Dim Debug As Long

'$Include:'../../../source/global/constants.bas'
sp = "@" ' Makes sequences easier to write

'$Include:'../../../source/utilities/const_eval.bi'
'$Include:'../../../source/utilities/ini-manager/ini.bi'
'$Include:'../../../source/utilities/s-buffer/simplebuffer.bi'
'$Include:'../../../source/utilities/hash.bi'
'$Include:'../../../source/utilities/type.bi'
'$Include:'../../../source/utilities/give_error.bi'

Dim tests(10) As String
tests(1) = "2@+@+@+@3"
tests(2) = "2@-@-@3"
tests(3) = "2@-@+@3@-@-@4"
tests(4) = "(@-@-@3@+@+@3@)@-@-@3"
tests(5) = "-@-@3"
tests(6) = "-@+@3"
tests(7) = "+@-@3"
tests(8) = "+@+@3"
tests(9) = "+@+@+@3"
tests(10) = "-@-@-@3"

For i = 1 To UBound(tests)
    Print "Test: "; Readable$(tests(i))
    Print "DWD: "; Readable$(DWD$(tests(i)))
Next i

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
