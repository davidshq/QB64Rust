' CSRLIN, POS, SCREEN and POINT: each result printed, and so part of the state.
Dim r As Long, c As Long
Locate 4, 7: r = CsrLin: c = Pos(0): Print r; c
Print CsrLin; Pos(0)
Print "abc"; Pos(0); CsrLin
Print "abc";: Print Pos(1); CsrLin * 2 + Pos(0)
Color 14, 1: Locate 8, 3: Print "Q";
Color 7, 0
Locate 10, 1
Print Screen(8, 3); Screen(8, 3, 0); Screen(8, 3, 1); Screen(1, 1); Screen(8, 4, 1)
Print Hex$(Screen(8, 3, 1)); Screen(8, 3) / 2; -Screen(8, 3); Screen(8, 3) - 100
r = Screen(8, 3): c = Screen(8, 3, 1)
Print r; c; Pos(0) + CsrLin
Locate 25, 80: Print "x";
r = CsrLin: c = Pos(0)
Locate 14, 1: Print r; c
Screen 12
PSet (5, 305), 9
Line (10, 310)-(20, 320), 12, BF
Print Point(5, 305); Point(15, 315); Point(100, 100); Point(-1, -1); Point(640, 5)
Print Point(0); Point(1); Point(2); Point(3)
PSet (30, 40)
Print Point(0); Point(1)
Print Hex$(Point(5, 305)); Point(5, 305) / 2; Point(15, 315) * 100000
r = Point(15, 315): Print r
Window (0, 0)-(100, 100)
PSet (50, 25)
Print Point(0); Point(1); Point(2); Point(3)
Window
Print CsrLin; Pos(0); Screen(1, 1); Screen(1, 2)
System
