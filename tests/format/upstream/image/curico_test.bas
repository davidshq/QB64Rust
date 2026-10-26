DefLng A-Z
Option _Explicit
$Console:Only
ChDir _StartDir$

Const TEST_FILE_FORMAT = "bmp"
Const TOLERANCE_LIMIT = 0

On Error GoTo error_handler

Restore file_list

Dim fileName As String: Read fileName

Do
    DoImageFile fileName
    Read fileName
Loop While Len(fileName)

System

error_handler:
Print "Error: "; _ErrorMessage$; " at line"; _ErrorLine;

Dim incErrFile As String: incErrFile = _InclErrorFile$
Dim incErrLine As Long: incErrLine = _InclErrorLine

If Len(incErrFile) Then
    Print "( "; incErrFile + " at line"; incErrLine; ")"
Else
    Print
End If

System

file_list:
Data "1bpp.ico"
Data "24bpp.ico"
Data "4bpp.cur"
Data "4bpp.ico"
Data "8bpp.ico"
Data "8bpp.cur"
Data "bard.ico"
Data "broom.ico"
Data "cat.ico"
Data "computer.ico"
Data "dumb1.ico"
Data "dumb2.ico"
Data "pencil.ico"
Data "please_wait.ico"
Data "question.ico"
Data "really_huge_duck.ico"
Data "sample3.ico"
Data "sample4.ico"
Data "soft_drink.ico"
Data "taco.ico"
Data "terra_globe.ico"
Data "thumbs_up.ico"
Data "volme_dialog.ico"
Data "word-icon.ico"
Data "x.ico"
Data ""

Sub PrintImageDetails (handle As Long, testFileName As String)
    Dim fullTestFileName As String: fullTestFileName = testFileName + "." + TEST_FILE_FORMAT

    Print "Testing against "; fullTestFileName; " ("; _Width(handle); "x"; _Height(handle); ")."
    '_SAVEIMAGE fullTestFileName, handle
    AssertImage2 handle, fullTestFileName, TOLERANCE_LIMIT

    Print
End Sub


Sub DoImageFile (testFileName As String)
    Dim fileName As String: fileName = testFileName

    Print "Loading image from storage "; fileName; " ... ";

    Dim h As Long: h = _LoadImage(fileName, 32)

    If h < -1 Then
        Print "done."

        PrintImageDetails h, testFileName

        _FreeImage h
    Else
        Print "failed!"
    End If
End Sub

'$Include:'../utilities/imageassert.bm'
'$Include:'../utilities/base64.bm'
