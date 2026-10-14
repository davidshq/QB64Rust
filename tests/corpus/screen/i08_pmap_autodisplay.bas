' PMAP with WINDOW and VIEW, each of its four functions, and _AUTODISPLAY: its value before and after _DISPLAY and
' after the statement, with a circle drawn while the screen was not shown at once (m3-images-fonts-mouse,
' task 2.4; verification\v31_b_images).
SCREEN 12
WINDOW (0, 0)-(100, 100)
LINE (10, 10)-(90, 90), 4, B
x = PMAP(50, 0): y = PMAP(50, 1): lx = PMAP(320, 2): ly = PMAP(240, 3)
WINDOW
VIEW (100, 100)-(300, 200)
WINDOW SCREEN (0, 0)-(10, 10)
LINE (1, 1)-(9, 9), 2, B
q0 = PMAP(5, 0): q1 = PMAP(5, 1): q2 = PMAP(100, 2): q3 = PMAP(50, 3)
VIEW
WINDOW
a0& = _AUTODISPLAY
_DISPLAY
a1& = _AUTODISPLAY
CIRCLE (500, 300), 50, 14
_AUTODISPLAY
a2& = _AUTODISPLAY
LOCATE 20, 1
PRINT x; y; lx; ly
PRINT q0; q1; q2; q3; PMAP(0, 0)
PRINT a0&; a1&; a2&
SYSTEM
