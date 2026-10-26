Option _Explicit
$Console:Only

ChDir _StartDir$

Const IMG_W = 320
Const IMG_H = 200
Const WXMIN = -0.5
Const WYMIN = -0.5
Const WXMAX = 0.5
Const WYMAX = 0.5

Dim img As Long: img = _NewImage(IMG_W, IMG_H, 32)
_Dest img
Cls , _RGB32(0, 0, 0)

Window (-1, 1)-(1, -1)

Dim VXMin As Long: VXMin = PMap(WXMIN, 0)
Dim VYMin As Long: VYMin = PMap(WYMIN, 1)
Dim VXMax As Long: VXMax = PMap(WXMAX, 0)
Dim VYMax As Long: VYMax = PMap(WYMAX, 1)

View Screen(VXMin, VYMin)-(VXMax, VYMax)
Window (-1, 1)-(1, -1)
View Screen(VXMin, VYMin)-(VXMax, VYMax), _RGB32(64), _RGB32(0, 255, 255)
Line (WXMIN, WYMIN)-(WXMAX, WYMAX), _RGB32(255, 255, 255)

'_SAVEIMAGE "window_view_screen_line.png", img

_Dest _Console
AssertImage2 img, "window_view_screen_line.png", 0

_FreeImage img
System

'$Include:'../utilities/imageassert.bm'
