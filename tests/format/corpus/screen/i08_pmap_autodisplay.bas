' PMAP with WINDOW and VIEW, each of its four functions, and _AUTODISPLAY: its value before and after _DISPLAY and
' after the statement, with a circle drawn while the screen was not shown at once (m3-images-fonts-mouse,
' task 2.4; verification\v31_b_images).
Screen 12
Window (0, 0)-(100, 100)
Line (10, 10)-(90, 90), 4, B
x = PMap(50, 0): y = PMap(50, 1): lx = PMap(320, 2): ly = PMap(240, 3)
Window
View (100, 100)-(300, 200)
Window Screen(0, 0)-(10, 10)
Line (1, 1)-(9, 9), 2, B
q0 = PMap(5, 0): q1 = PMap(5, 1): q2 = PMap(100, 2): q3 = PMap(50, 3)
View
Window
a0& = _AutoDisplay
_Display
a1& = _AutoDisplay
Circle (500, 300), 50, 14
_AutoDisplay
a2& = _AutoDisplay
Locate 20, 1
Print x; y; lx; ly
Print q0; q1; q2; q3; PMap(0, 0)
Print a0&; a1&; a2&
System
