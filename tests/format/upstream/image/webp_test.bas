Option _Explicit
$Console:Only
ChDir _StartDir$

Const TEST_IMAGE_FORMAT = "bmp"
Const TEST_IMAGE_01 = "1"
Const TEST_IMAGE_02 = "2"
Const TEST_IMAGE_03 = "3"
Const TEST_IMAGE_04 = "4"
Const TEST_IMAGE_05 = "5"
Const TOLERANCE_LIMIT = 0

DoImageFile TEST_IMAGE_01
DoImageFile TEST_IMAGE_02
DoImageFile TEST_IMAGE_03
DoImageFile TEST_IMAGE_04
DoImageFile TEST_IMAGE_05

System


Sub PrintImageDetails (handle As Long, testFileName As String)
    _Dest handle

    Dim iWidth As Long: iWidth = _Width(handle)
    Dim iHeight As Long: iHeight = _Height(handle)

    _Dest _Console

    Dim fullTestFileName As String: fullTestFileName = testFileName + "." + TEST_IMAGE_FORMAT

    Print "Testing against "; fullTestFileName; " ("; iWidth; "x"; iHeight; ")."
    '_SAVEIMAGE testFileName, handle, TEST_IMAGE_FORMAT
    AssertImage2 handle, fullTestFileName, TOLERANCE_LIMIT

    Print
End Sub


Sub DoImageFile (testFileName As String)
    Dim fileName As String: fileName = testFileName + ".webp"

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
