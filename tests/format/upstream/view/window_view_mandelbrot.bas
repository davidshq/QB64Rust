DefLng A-Z
Option _Explicit
$Console:Only

ChDir _StartDir$

Const MAXLOOP = 30, MAXSIZE = 1000000
Const WLeft = -1000, WRight = 250, WTop = 625, WBottom = -625
Const VLeft = 110, VRight = 529, VTop = 5, VBottom = 179
Const ColorRange = 15

Dim img As Long: img = _NewImage(640, 200, 8)
_Dest img

View (VLeft, VTop)-(VRight, VBottom), 0, ColorRange
Window (WLeft, WTop)-(WRight, WBottom)

Dim XLength As Long: XLength = VRight - VLeft
Dim YLength As Long: YLength = VBottom - VTop
Dim ColorWidth As Long: ColorWidth = MAXLOOP \ ColorRange

Dim As Long x, y
For y = 0 To YLength
    Dim LogicY As Long: LogicY = PMap(y, 3)
    PSet (WLeft, LogicY)
    Dim OldColor As Long: OldColor = 0

    For x = 0 To XLength
        Dim LogicX As Long: LogicX = PMap(x, 2)
        Dim MandelX As Long: MandelX = LogicX
        Dim MandelY As Long: MandelY = LogicY

        Dim i As Long
        For i = 1 To MAXLOOP
            Dim RealNum As Long: RealNum = MandelX * MandelX
            Dim ImagNum As Long: ImagNum = MandelY * MandelY
            If (RealNum + ImagNum) >= MAXSIZE Then Exit For
            MandelY = (MandelX * MandelY) \ 250 + LogicY
            MandelX = (RealNum - ImagNum) \ 500 + LogicX
        Next i

        Dim PColor As Long: PColor = i \ ColorWidth

        If PColor <> OldColor Then
            Line -(LogicX, LogicY), (ColorRange - OldColor)
            OldColor = PColor
        End If
    Next x

    Line -(LogicX, LogicY), (ColorRange - OldColor)
Next y

View
Window

'_SAVEIMAGE "window_view_mandelbrot.png", img

_Dest _Console
AssertImage2 img, "window_view_mandelbrot.png", 0

_FreeImage img
System

'$Include:'../utilities/imageassert.bm'

