Option _Explicit
$Console:Only
ChDir _StartDir$

Const TEST_IMAGE_FORMAT = "bmp"
Const TEST_IMAGE_01 = "16color1"
Const TEST_IMAGE_02 = "16color2"
Const TEST_IMAGE_03 = "blood4"
Const TEST_IMAGE_04 = "fm_d0010"
Const TEST_IMAGE_05 = "gmarbles"
Const TEST_IMAGE_06 = "lena"
Const TEST_IMAGE_07 = "lena10"
Const TEST_IMAGE_08 = "lena2"
Const TEST_IMAGE_09 = "lena3"
Const TEST_IMAGE_10 = "lena4"
Const TEST_IMAGE_11 = "lena5"
Const TEST_IMAGE_13 = "lena8"
Const TEST_IMAGE_14 = "lena9"
Const TEST_IMAGE_15 = "marbles"
Const TEST_IMAGE_16 = "prey0013"
Const TEST_IMAGE_17 = "sample_1280_853"
Const TEST_IMAGE_18 = "sample_1920_1280"
Const TEST_IMAGE_19 = "sample_640_426"
Const TEST_IMAGE_20 = "swpic2"
Const TEST_IMAGE_21 = "title"
Const TOLERANCE_LIMIT = 0

DoImageFile TEST_IMAGE_01
DoImageFile TEST_IMAGE_02
DoImageFile TEST_IMAGE_03
DoImageFile TEST_IMAGE_04
DoImageFile TEST_IMAGE_05
DoImageFile TEST_IMAGE_06
DoImageFile TEST_IMAGE_07
DoImageFile TEST_IMAGE_08
DoImageFile TEST_IMAGE_09
DoImageFile TEST_IMAGE_10
DoImageFile TEST_IMAGE_11
DoImageFile TEST_IMAGE_13
DoImageFile TEST_IMAGE_14
DoImageFile TEST_IMAGE_15
DoImageFile TEST_IMAGE_16
DoImageFile TEST_IMAGE_17
DoImageFile TEST_IMAGE_18
DoImageFile TEST_IMAGE_19
DoImageFile TEST_IMAGE_20
DoImageFile TEST_IMAGE_21

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
    Dim fileName As String: fileName = testFileName + ".pcx"

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
