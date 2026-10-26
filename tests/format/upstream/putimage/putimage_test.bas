Option _Explicit
$Console:Only

ChDir _StartDir$

Type TestStats
    total As Integer
    failed As Integer
End Type

Dim Shared stats As TestStats
Dim Shared errCaught As _Byte

Dim Shared srcText As Long: srcText = _NewImage(20, 10, 0)
PrepareTextSource srcText

TestTextToTextBasic
TestTextToTextReversed
TestTextToTextMirrored
TestTextToTextFlipped
TestTextToTextClipping

TestTextToGraphics32Basic
TestTextToGraphics32ViewClip
TestTextToGraphics32WindowNoScale
TestTextToGraphics32Reversed
TestTextToGraphics32Mirrored
TestTextToGraphics32Flipped
TestTextToGraphics32Clipping
TestGraphicsToGraphics32ViewClip
TestGraphicsToGraphics32WindowNoScale
TestGraphicsToGraphics32SourceViewClip

TestTextToGraphics8Basic
TestTextToGraphics8Mirrored
TestTextToGraphics8Flipped
TestTextToGraphics4Basic
TestTextToGraphics4Mirrored
TestTextToGraphics4Flipped

TestTextToTextViewPrint
TestTextToTextScaleIllegal
TestTextToGraphicsScaleIllegal

TestTextToGraphics2Illegal
TestGraphicsToTextIllegal

TestTextToGraphics32TTFFont

_FreeImage srcText

If stats.failed = 0 Then
    Print "ALL TESTS PASSED"; stats.total
Else
    Print "TESTS FAILED"; stats.failed; "of"; stats.total
End If

System

err_text_to_gfx2:
errCaught = _TRUE
Resume Next

err_gfx_to_text:
errCaught = _TRUE
Resume Next

err_text_to_text_scale:
errCaught = _TRUE
Resume Next

err_text_to_gfx_scale:
errCaught = _TRUE
Resume Next

Sub ReportCheck (testName As String, condition As Integer)
    _Dest _Console
    stats.total = stats.total + 1
    If condition Then
        Print "PASS: "; testName
    Else
        stats.failed = stats.failed + 1
        Print "FAIL: "; testName
    End If
End Sub

Sub PrepareTextSource (img As Long)
    Dim oldDest As Long: oldDest = _Dest

    _Dest img
    Cls

    Color 15, 1
    Locate 1, 1: Print "ABCDEFGHIJKLMNOPQRST";
    Color 14, 4
    Locate 2, 1: Print "abcdefghijklmnopqrst";
    Color 10, 2
    Locate 3, 1: Print "01234567890123456789";
    Color 12, 3
    Locate 4, 1: Print "!@#$%^&*()_+[]{}<>?/";

    Color 11, 5
    Locate 6, 4: Print "BOX";
    Color 15, 6
    Locate 7, 4: Print "REV";
    Color 0, 7
    Locate 8, 4: Print "INV";

    '_CLEARCOLOR _NONE, img
    _ClearColor Asc("?") + 256 * (12 + 16 * 3), img

    _Dest oldDest
End Sub

Sub PrepareTextDestination (img As Long)
    Dim oldDest As Long: oldDest = _Dest

    _Dest img
    Cls

    Color 7, 0
    Dim y As Long
    For y = 1 To _Height(img)
        Locate y, 1
        Print String$(_Width(img), ".");
    Next

    _Dest oldDest
End Sub

Sub PrepareGraphicsDestination32 (img As Long)
    Dim oldDest As Long: oldDest = _Dest

    _Dest img
    Cls , _RGB32(0, 0, 0)
    Line (0, 0)-(_Width(img) - 1, _Height(img) - 1), _RGB32(0, 0, 0), BF

    _Dest oldDest
End Sub

Sub PrepareGraphicsSource32 (img As Long)
    Dim oldDest As Long: oldDest = _Dest

    _Dest img
    Cls , _RGB32(0, 0, 0)
    Line (0, 0)-(_Width(img) - 1, _Height(img) - 1), _RGB32(255, 64, 32), BF
    Line (8, 8)-(_Width(img) - 9, _Height(img) - 9), _RGB32(32, 192, 255), BF
    Line (16, 16)-(_Width(img) - 17, _Height(img) - 17), _RGB32(255, 255, 64), BF

    _Dest oldDest
End Sub

Sub PrepareGraphicsDestinationIndexed (img As Long)
    Dim oldDest As Long
    oldDest = _Dest

    _Dest img
    Cls , 0
    Line (0, 0)-(_Width(img) - 1, _Height(img) - 1), 0, BF

    _Dest oldDest
End Sub

Function TextCell~% (img As Long, cx As Long, cy As Long)
    ' DOES NOT CLIP!
    Dim p As _MEM: p = _MemImage(img)
    TextCell = _MemGet(p, p.OFFSET + ((cy - 1) * _Width(img) + (cx - 1)) * _SIZE_OF_INTEGER, _Unsigned Integer)
    _MemFree p
End Function

Function Pixel32~& (img As Long, x As Long, y As Long)
    ' DOES NOT CLIP!
    Dim p As _MEM: p = _MemImage(img)
    Pixel32 = _MemGet(p, p.OFFSET + (y * _Width(img) + x) * _SIZE_OF_LONG, _Unsigned Long)
    _MemFree p
End Function

Function PixelIndex~%% (img As Long, x As Long, y As Long)
    ' DOES NOT CLIP!
    Dim p As _MEM: p = _MemImage(img)
    PixelIndex = _MemGet(p, p.OFFSET + (y * _Width(img) + x), _Unsigned _Byte)
    _MemFree p
End Function

Function CountNonBlack32~& (img As Long, x1 As Long, y1 As Long, x2 As Long, y2 As Long)
    Dim x As Long
    Dim y As Long
    Dim c As _Unsigned Long
    Dim count As _Unsigned Long

    For y = y1 To y2
        For x = x1 To x2
            c = Pixel32(img, x, y)
            If _Red32(c) <> 0 _OrElse _Green32(c) <> 0 _OrElse _Blue32(c) <> 0 Then count = count + 1
        Next
    Next

    CountNonBlack32 = count
End Function

Function CountNonBlackOutsideRect32~& (img As Long, x1 As Long, y1 As Long, x2 As Long, y2 As Long)
    Dim x As Long
    Dim y As Long
    Dim c As _Unsigned Long
    Dim count As _Unsigned Long

    For y = 0 To _Height(img) - 1
        For x = 0 To _Width(img) - 1
            If x >= x1 _AndAlso x <= x2 _AndAlso y >= y1 _AndAlso y <= y2 Then
                ' inside rectangle
            Else
                c = Pixel32(img, x, y)
                If _Red32(c) <> 0 _OrElse _Green32(c) <> 0 _OrElse _Blue32(c) <> 0 Then count = count + 1
            End If
        Next
    Next

    CountNonBlackOutsideRect32 = count
End Function

Function SumRegion32~&& (img As Long, x1 As Long, y1 As Long, x2 As Long, y2 As Long)
    Dim x As Long
    Dim y As Long
    Dim s As _Unsigned _Integer64

    For y = y1 To y2
        For x = x1 To x2
            s = s + Pixel32(img, x, y)
        Next
    Next

    SumRegion32 = s
End Function

Function CountNonZeroIndexed~& (img As Long, x1 As Long, y1 As Long, x2 As Long, y2 As Long)
    Dim x As Long
    Dim y As Long
    Dim count As _Unsigned Long

    For y = y1 To y2
        For x = x1 To x2
            If PixelIndex(img, x, y) <> 0 Then count = count + 1
        Next
    Next

    CountNonZeroIndexed = count
End Function

Function RegionsEqualText%% (srcImg As Long, srcX As Long, srcY As Long, dstImg As Long, dstX As Long, dstY As Long, w As Long, h As Long, dstStepX As Long, dstStepY As Long)
    Dim x As Long
    Dim y As Long
    Dim sx As Long
    Dim sy As Long
    Dim dx As Long
    Dim dy As Long

    RegionsEqualText = _TRUE

    For y = 0 To h - 1
        For x = 0 To w - 1
            sx = srcX + x
            sy = srcY + y
            dx = dstX + x * dstStepX
            dy = dstY + y * dstStepY

            If TextCell(srcImg, sx, sy) <> TextCell(dstImg, dx, dy) Then
                RegionsEqualText = _FALSE
                Exit Function
            End If
        Next
    Next
End Function

Function RegionsEqual32%% (srcImg As Long, srcX As Long, srcY As Long, dstImg As Long, dstX As Long, dstY As Long, w As Long, h As Long, dstStepX As Long, dstStepY As Long)
    Dim x As Long
    Dim y As Long
    Dim sx As Long
    Dim sy As Long
    Dim dx As Long
    Dim dy As Long

    RegionsEqual32 = _TRUE

    For y = 0 To h - 1
        For x = 0 To w - 1
            sx = srcX + x
            sy = srcY + y
            dx = dstX + x * dstStepX
            dy = dstY + y * dstStepY

            If Pixel32(srcImg, sx, sy) <> Pixel32(dstImg, dx, dy) Then
                RegionsEqual32 = _FALSE
                Exit Function
            End If
        Next
    Next
End Function

Function RegionsEqualIndexed%% (srcImg As Long, srcX As Long, srcY As Long, dstImg As Long, dstX As Long, dstY As Long, w As Long, h As Long, dstStepX As Long, dstStepY As Long)
    Dim x As Long
    Dim y As Long
    Dim sx As Long
    Dim sy As Long
    Dim dx As Long
    Dim dy As Long

    RegionsEqualIndexed = _TRUE

    For y = 0 To h - 1
        For x = 0 To w - 1
            sx = srcX + x
            sy = srcY + y
            dx = dstX + x * dstStepX
            dy = dstY + y * dstStepY

            If PixelIndex(srcImg, sx, sy) <> PixelIndex(dstImg, dx, dy) Then
                RegionsEqualIndexed = _FALSE
                Exit Function
            End If
        Next
    Next
End Function

Sub TestTextToTextBasic
    Dim dst As Long: dst = _NewImage(30, 16, 0)
    PrepareTextDestination dst

    _PutImage (4, 3), srcText, dst
    '_SAVEIMAGE "TestTextToTextBasic", dst

    Dim ok As _Byte: ok = _TRUE
    If TextCell(dst, 5, 4) <> TextCell(srcText, 1, 1) Then ok = _FALSE
    If TextCell(dst, 10, 4) <> TextCell(srcText, 6, 1) Then ok = _FALSE
    If TextCell(dst, 7, 6) <> TextCell(srcText, 3, 3) Then ok = _FALSE
    If TextCell(dst, 24, 13) <> TextCell(srcText, 20, 10) Then ok = _FALSE

    ReportCheck "text->text basic", ok
    _FreeImage dst
End Sub

Sub TestTextToTextReversed
    Dim dst As Long: dst = _NewImage(30, 16, 0)
    PrepareTextDestination dst

    _PutImage (14, 7)-(9, 5), srcText, dst, (5, 2)-(0, 0)
    '_SAVEIMAGE "TestTextToTextReversed", dst

    Dim ok As _Byte: ok = _TRUE
    If TextCell(dst, 15, 8) <> TextCell(srcText, 6, 3) Then ok = _FALSE
    If TextCell(dst, 10, 6) <> TextCell(srcText, 1, 1) Then ok = _FALSE
    If TextCell(dst, 14, 8) <> TextCell(srcText, 5, 3) Then ok = _FALSE
    If TextCell(dst, 15, 7) <> TextCell(srcText, 6, 2) Then ok = _FALSE

    ReportCheck "text->text reversed rect", ok
    _FreeImage dst
End Sub

Sub TestTextToTextMirrored
    Dim ref As Long: ref = _NewImage(30, 16, 0)
    PrepareTextDestination ref
    Dim dst As Long: dst = _NewImage(30, 16, 0)
    PrepareTextDestination dst

    _PutImage (5, 3), srcText, ref
    '_SAVEIMAGE "TestTextToTextMirroredRef", ref

    _PutImage (24, 3)-(5, 12), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToTextMirroredDst", dst

    Dim ok As _Byte: ok = RegionsEqualText(ref, 6, 4, dst, 25, 4, 20, 10, -1, 1)
    If TextCell(dst, 25, 4) <> TextCell(srcText, 1, 1) Then ok = _FALSE
    If TextCell(dst, 6, 4) <> TextCell(srcText, 20, 1) Then ok = _FALSE

    ReportCheck "text->text mirrored", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToTextFlipped
    Dim ref As Long: ref = _NewImage(30, 16, 0)
    PrepareTextDestination ref
    Dim dst As Long: dst = _NewImage(30, 16, 0)
    PrepareTextDestination dst

    _PutImage (5, 3), srcText, ref
    '_SAVEIMAGE "TestTextToTextFlippedRef", ref

    _PutImage (24, 12)-(5, 3), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToTextFlippedDst", dst

    Dim ok As _Byte: ok = RegionsEqualText(ref, 6, 4, dst, 25, 13, 20, 10, -1, -1)
    If TextCell(dst, 6, 4) <> TextCell(srcText, 20, 10) Then ok = _FALSE
    If TextCell(dst, 25, 13) <> TextCell(srcText, 1, 1) Then ok = _FALSE

    ReportCheck "text->text flipped", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToTextClipping
    Dim dst As Long: dst = _NewImage(30, 16, 0)
    PrepareTextDestination dst

    _PutImage (-3, 0), srcText, dst
    '_SAVEIMAGE "TestTextToTextClipping", dst

    Dim ok As _Byte: ok = _TRUE
    If TextCell(dst, 1, 1) <> TextCell(srcText, 4, 1) Then ok = _FALSE
    If TextCell(dst, 2, 1) <> TextCell(srcText, 5, 1) Then ok = _FALSE

    ReportCheck "text->text clipping", ok
    _FreeImage dst
End Sub

Sub TestTextToGraphics32Basic
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    _PutImage (16, 16), srcText, dst
    '_SAVEIMAGE "TestTextToGraphics32Basic", dst

    Dim nonBlack As _Unsigned Long: nonBlack = CountNonBlack32(dst, 16, 16, 16 + 20 * 8 - 1, 16 + 10 * 16 - 1)
    Dim ok As _Byte: ok = (nonBlack > 0)
    If Pixel32(dst, 0, 0) <> _RGB32(0, 0, 0) Then ok = _FALSE

    ReportCheck "text->graphics 32bpp basic", ok
    _FreeImage dst
End Sub

Sub TestTextToGraphics32ViewClip
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    _Dest dst
    View (40, 40)-(120, 120)
    _PutImage (0, 0), srcText
    View
    '_SAVEIMAGE "TestTextToGraphics32ViewClip", dst

    Dim ok As _Byte: ok = _TRUE
    If CountNonBlack32(dst, 40, 40, 120, 120) <= 0 Then ok = _FALSE
    If CountNonBlackOutsideRect32(dst, 40, 40, 120, 120) <> 0 Then ok = _FALSE

    ReportCheck "text->graphics 32bpp VIEW clip", ok
    _FreeImage dst
End Sub

Sub TestTextToGraphics32WindowNoScale
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    _Dest dst
    View (40, 40)-(199, 199)
    Window Screen(0, 0)-(159, 159)
    _PutImage (0, 0), srcText
    Window
    View
    '_SAVEIMAGE "TestTextToGraphics32WindowNoScale", dst

    Dim ok As _Byte: ok = _TRUE
    If CountNonBlack32(dst, 40, 40, 199, 199) <= 0 Then ok = _FALSE
    If CountNonBlackOutsideRect32(dst, 40, 40, 199, 199) <> 0 Then ok = _FALSE

    ReportCheck "text->graphics 32bpp WINDOW", ok
    _FreeImage dst
End Sub

Sub TestTextToGraphics32Reversed
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst
    Dim ref As Long: ref = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 ref


    _PutImage (100, 80)-(147, 111), srcText, ref, (0, 0)-(5, 1)
    '_SAVEIMAGE "TestTextToGraphics32ReversedRef", ref

    _PutImage (200, 120)-(153, 89), srcText, dst, (5, 1)-(0, 0)
    '_SAVEIMAGE "TestTextToGraphics32ReversedDst", dst

    Dim nonBlackRef As _Unsigned Long: nonBlackRef = CountNonBlack32(ref, 100, 80, 147, 111)
    Dim nonBlackDst As _Unsigned Long: nonBlackDst = CountNonBlack32(dst, 153, 89, 200, 120)

    Dim sumRef As _Unsigned _Integer64: sumRef = SumRegion32(ref, 100, 80, 147, 111)
    Dim sumDst As _Unsigned _Integer64: sumDst = SumRegion32(dst, 153, 89, 200, 120)

    Dim ok As _Byte: ok = _TRUE
    If nonBlackRef <= 0 Then ok = _FALSE
    If nonBlackDst <> nonBlackRef Then ok = _FALSE
    If sumDst <> sumRef Then ok = _FALSE

    ReportCheck "text->graphics 32bpp reversed rect", ok

    _FreeImage ref
    _FreeImage dst
End Sub

Sub TestTextToGraphics32Mirrored
    Dim ref As Long: ref = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 ref
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    _PutImage (20, 20), srcText, ref
    '_SAVEIMAGE "TestTextToGraphics32MirroredRef", ref

    _PutImage (179, 20)-(20, 179), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToGraphics32MirroredDst", dst

    Dim ok As _Byte: ok = RegionsEqual32(ref, 20, 20, dst, 179, 20, 160, 160, -1, 1)

    ReportCheck "text->graphics 32bpp mirrored", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToGraphics32Flipped
    Dim ref As Long: ref = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 ref
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    _PutImage (20, 20), srcText, ref
    '_SAVEIMAGE "TestTextToGraphics32FlippedRef", ref

    _PutImage (179, 179)-(20, 20), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToGraphics32FlippedDst", dst

    Dim ok As _Byte: ok = RegionsEqual32(ref, 20, 20, dst, 179, 179, 160, 160, -1, -1)

    ReportCheck "text->graphics 32bpp flipped", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToGraphics32Clipping
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst


    _PutImage (-12, -10), srcText, dst
    '_SAVEIMAGE "TestTextToGraphics32Clipping", dst

    Dim ok As _Byte: ok = (CountNonBlack32(dst, 0, 0, 80, 80) > 0)

    ReportCheck "text->graphics 32bpp clipping", ok
    _FreeImage dst
End Sub

Sub TestGraphicsToGraphics32ViewClip
    Dim src As Long: src = _NewImage(80, 80, 32)
    PrepareGraphicsSource32 src
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    _Dest dst
    View (40, 40)-(99, 99)
    _PutImage (0, 0), src, dst
    View
    '_SAVEIMAGE "TestGraphicsToGraphics32ViewClip", dst

    Dim ok As _Byte: ok = _TRUE
    If CountNonBlack32(dst, 40, 40, 99, 99) <= 0 Then ok = _FALSE
    If CountNonBlackOutsideRect32(dst, 40, 40, 99, 99) <> 0 Then ok = _FALSE
    If Pixel32(dst, 40, 40) <> _RGB32(255, 64, 32) Then ok = _FALSE
    If Pixel32(dst, 48, 48) <> _RGB32(32, 192, 255) Then ok = _FALSE
    If Pixel32(dst, 56, 56) <> _RGB32(255, 255, 64) Then ok = _FALSE
    If Pixel32(dst, 99, 99) <> _RGB32(255, 255, 64) Then ok = _FALSE

    ReportCheck "graphics->graphics 32bpp VIEW clip", ok
    _FreeImage dst
    _FreeImage src
End Sub

Sub TestGraphicsToGraphics32WindowNoScale
    Dim src As Long: src = _NewImage(80, 80, 32)
    PrepareGraphicsSource32 src
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst
    Dim ref As Long: ref = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 ref

    _PutImage (80, 80), src, ref

    _Dest dst
    View (40, 40)-(199, 199)
    Window Screen(0, 0)-(159, 159)
    _PutImage (40, 40), src, dst
    Window
    View
    '_SAVEIMAGE "TestGraphicsToGraphics32WindowNoScale", dst

    Dim ok As _Byte: ok = _TRUE
    If RegionsEqual32(ref, 80, 80, dst, 80, 80, 80, 80, 1, 1) = _FALSE Then ok = _FALSE
    If CountNonBlackOutsideRect32(dst, 80, 80, 159, 159) <> 0 Then ok = _FALSE

    ReportCheck "graphics->graphics 32bpp WINDOW", ok
    _FreeImage ref
    _FreeImage dst
    _FreeImage src
End Sub

Sub TestGraphicsToGraphics32SourceViewClip
    Dim src As Long: src = _NewImage(80, 80, 32)
    PrepareGraphicsSource32 src
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    _Dest src
    View (20, 20)-(59, 59)
    _PutImage (30, 30), src, dst
    View
    '_SAVEIMAGE "TestGraphicsToGraphics32SourceViewClip", dst

    Dim ok As _Byte: ok = _TRUE
    If CountNonBlack32(dst, 50, 50, 89, 89) <= 0 Then ok = _FALSE
    If CountNonBlackOutsideRect32(dst, 50, 50, 89, 89) <> 0 Then ok = _FALSE

    ReportCheck "graphics->graphics 32bpp source VIEW", ok
    _FreeImage dst
    _FreeImage src
End Sub

Sub TestTextToGraphics8Basic
    Dim dst As Long: dst = _NewImage(320, 200, 256)
    PrepareGraphicsDestinationIndexed dst

    _PutImage (24, 24), srcText, dst
    '_SAVEIMAGE "TestTextToGraphics8Basic", dst

    Dim ok As _Byte: ok = (CountNonZeroIndexed(dst, 24, 24, 24 + 20 * 8 - 1, 24 + 10 * 16 - 1) > 0)
    ReportCheck "text->graphics 8bpp basic", ok

    _FreeImage dst
End Sub

Sub TestTextToGraphics8Mirrored
    Dim ref As Long: ref = _NewImage(320, 200, 256)
    PrepareGraphicsDestinationIndexed ref
    Dim dst As Long: dst = _NewImage(320, 200, 256)
    PrepareGraphicsDestinationIndexed dst

    _PutImage (20, 20), srcText, ref
    '_SAVEIMAGE "TestTextToGraphics8MirroredRef", ref

    _PutImage (179, 20)-(20, 179), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToGraphics8MirroredDst", dst

    Dim ok As _Byte: ok = RegionsEqualIndexed(ref, 20, 20, dst, 179, 20, 160, 160, -1, 1)

    ReportCheck "text->graphics 8bpp mirrored", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToGraphics8Flipped
    Dim ref As Long: ref = _NewImage(320, 200, 256)
    Dim dst As Long: dst = _NewImage(320, 200, 256)

    PrepareGraphicsDestinationIndexed ref
    PrepareGraphicsDestinationIndexed dst

    _PutImage (20, 20), srcText, ref
    '_SAVEIMAGE "TestTextToGraphics8FlippedRef", ref

    _PutImage (179, 179)-(20, 20), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToGraphics8FlippedDst", dst

    Dim ok As _Byte: ok = RegionsEqualIndexed(ref, 20, 20, dst, 179, 179, 160, 160, -1, -1)

    ReportCheck "text->graphics 8bpp flipped", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToGraphics4Basic
    Dim dst As Long: dst = _NewImage(320, 200, 12)
    PrepareGraphicsDestinationIndexed dst

    _PutImage (24, 24), srcText, dst
    '_SAVEIMAGE "TestTextToGraphics4Basic", dst

    Dim ok As _Byte: ok = (CountNonZeroIndexed(dst, 24, 24, 24 + 20 * 8 - 1, 24 + 10 * 16 - 1) > 0)
    ReportCheck "text->graphics 4bpp basic", ok

    _FreeImage dst
End Sub

Sub TestTextToGraphics4Mirrored
    Dim ref As Long: ref = _NewImage(320, 200, 12)
    PrepareGraphicsDestinationIndexed ref
    Dim dst As Long: dst = _NewImage(320, 200, 12)
    PrepareGraphicsDestinationIndexed dst

    _PutImage (20, 20), srcText, ref
    '_SAVEIMAGE "TestTextToGraphics4MirroredRef", ref

    _PutImage (179, 20)-(20, 179), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToGraphics4MirroredDst", dst

    Dim ok As _Byte: ok = RegionsEqualIndexed(ref, 20, 20, dst, 179, 20, 160, 160, -1, 1)

    ReportCheck "text->graphics 4bpp mirrored", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToGraphics4Flipped
    Dim ref As Long: ref = _NewImage(320, 200, 12)
    PrepareGraphicsDestinationIndexed ref
    Dim dst As Long: dst = _NewImage(320, 200, 12)
    PrepareGraphicsDestinationIndexed dst

    _PutImage (20, 20), srcText, ref
    '_SAVEIMAGE "TestTextToGraphics4FlippedRef", ref

    _PutImage (179, 179)-(20, 20), srcText, dst, (0, 0)-(19, 9)
    '_SAVEIMAGE "TestTextToGraphics4FlippedDst", dst

    Dim ok As _Byte: ok = RegionsEqualIndexed(ref, 20, 20, dst, 179, 179, 160, 160, -1, -1)

    ReportCheck "text->graphics 4bpp flipped", ok

    _FreeImage dst
    _FreeImage ref
End Sub

Sub TestTextToTextViewPrint
    Dim dst As Long: dst = _NewImage(30, 16, 0)
    PrepareTextDestination dst
    Dim ref As Long: ref = _NewImage(30, 16, 0)
    PrepareTextDestination ref

    _Dest dst
    View Print 6 To 10
    _PutImage (0, 0), srcText
    View Print
    '_SAVEIMAGE "TestTextToTextViewPrint", dst

    Dim ok As _Byte: ok = _TRUE
    If TextCell(dst, 1, 2) <> TextCell(ref, 1, 2) Then ok = _FALSE
    If TextCell(dst, 1, 6) <> TextCell(srcText, 1, 6) Then ok = _FALSE
    If TextCell(dst, 20, 10) <> TextCell(srcText, 20, 10) Then ok = _FALSE
    If TextCell(dst, 1, 11) <> TextCell(ref, 1, 11) Then ok = _FALSE

    ReportCheck "text->text VIEW PRINT", ok

    _FreeImage ref
    _FreeImage dst
End Sub

Sub TestTextToTextScaleIllegal
    Dim dst As Long: dst = _NewImage(30, 16, 0)
    PrepareTextDestination dst

    errCaught = _FALSE
    On Error GoTo err_text_to_text_scale
    _PutImage (0, 0)-(29, 15), srcText, dst
    On Error GoTo 0

    ReportCheck "text->text scale illegal", errCaught
    _FreeImage dst
End Sub

Sub TestTextToGraphicsScaleIllegal
    Dim dst As Long: dst = _NewImage(320, 200, 32)
    PrepareGraphicsDestination32 dst

    errCaught = _FALSE
    On Error GoTo err_text_to_gfx_scale
    _PutImage (0, 0)-(79, 79), srcText, dst
    On Error GoTo 0

    ReportCheck "text->graphics scale illegal", errCaught
    _FreeImage dst
End Sub

Sub TestTextToGraphics2Illegal
    Dim dst As Long: dst = _NewImage(320, 200, 1)
    PrepareGraphicsDestinationIndexed dst

    errCaught = _FALSE
    On Error GoTo err_text_to_gfx2
    _PutImage (0, 0), srcText, dst
    On Error GoTo 0

    ReportCheck "text->graphics 2bpp illegal", errCaught
    _FreeImage dst
End Sub

Sub TestGraphicsToTextIllegal
    Dim src As Long: src = _NewImage(64, 64, 32)
    Dim dst As Long: dst = _NewImage(30, 16, 0)

    PrepareTextDestination dst
    _Dest src
    Cls , _RGB32(0, 0, 0)
    Line (0, 0)-(63, 63), _RGB32(255, 0, 0), BF

    errCaught = _FALSE
    On Error GoTo err_gfx_to_text
    _PutImage (1, 1), src, dst
    On Error GoTo 0

    ReportCheck "graphics->text illegal", errCaught

    _FreeImage dst
    _FreeImage src
End Sub

Sub TestTextToGraphics32TTFFont
    Dim fnt As Long: fnt = _LoadFont("../font/LiberationMono-Regular.ttf", 16, "monospace")
    If fnt <= 0 Then
        ReportCheck "text->graphics 32bpp TTF font", _FALSE
        Exit Sub
    End If

    Dim fW As Integer: fW = _FontWidth(fnt)
    Dim fH As Integer: fH = _FontHeight(fnt)
    Dim src As Long: src = _NewImage(10, 4, 0)
    Dim oldDest As Long: oldDest = _Dest

    _Dest src
    _Font fnt
    Color 15, 1
    Cls
    Locate 1, 1: Print "ABCDEFGHIJ";
    Locate 2, 1: Print "abcdefghij";
    _Dest oldDest

    Dim expectedW As Long: expectedW = 10 * fW
    Dim expectedH As Long: expectedH = 4 * fH
    Dim dst As Long: dst = _NewImage(expectedW + 40, expectedH + 40, 32)
    PrepareGraphicsDestination32 dst

    _PutImage (20, 20), src, dst
    '_SAVEIMAGE "TestTextToGraphics32TTFFont", dst

    Dim ok As _Byte: ok = _TRUE
    If CountNonBlack32(dst, 20, 20, 20 + expectedW - 1, 20 + expectedH - 1) <= 0 Then ok = _FALSE
    If CountNonBlackOutsideRect32(dst, 20, 20, 20 + expectedW - 1, 20 + expectedH - 1) <> 0 Then ok = _FALSE

    ReportCheck "text->graphics 32bpp TTF font", ok

    _FreeImage src
    _FreeImage dst
    _FreeFont fnt
End Sub
