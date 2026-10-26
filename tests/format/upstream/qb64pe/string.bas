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

Dim test As String

test = Chr$(34) + "foobar" + Chr$(34) + ",6"

Dim num As ParseNum, eval As String

eval = Evaluate_Expression$(test, num)

Print "eval result: " + eval
Print "num.s: " + num.s


test = test + "@+@" + test

eval = Evaluate_Expression$(test, num)

Print "eval result: " + eval
Print "num.s: " + num.s

test = test + "@+@" + Chr$(34) + "test\034" + Chr$(34) + ",5"

eval = Evaluate_Expression$(test, num)

Print "eval result: " + eval
Print "num.s: " + num.s

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
