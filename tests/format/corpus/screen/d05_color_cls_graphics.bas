' COLOR and CLS in graphics modes: the colour drawing statements take when none is given, CLS with a viewport.
Screen 7
Line (300, 180)-(319, 199), 5, BF
Cls , 4
Color 3
Print "cyan"
Line (0, 20)-(50, 30)
Color 14, 1
Print "yellow, background 1"
PSet (60, 25)
View (100, 50)-(200, 150), 2, 15
Line (10, 10)-(40, 40), 4, BF
Cls 1
PSet (5, 5)
View Print 10 To 20
Print "text viewport"
Cls 2
Print "after CLS 2"
View Print
View (220, 100)-(300, 180), 5, 15
Cls
View
Line (250, 150)-(310, 190), , B
System
