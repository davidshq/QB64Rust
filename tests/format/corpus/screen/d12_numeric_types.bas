' Arguments of every numeric type: a coordinate, a colour, a radius, a row and a screen mode, floats that round.
Dim b As _Byte, ub As _Unsigned _Byte, i As Integer, ui As _Unsigned Integer, l As Long, ul As _Unsigned Long
Dim q As _Integer64, uq As _Unsigned _Integer64, s As Single, d As Double, f As _Float
s = 7.4
Screen s
Cls 1.4, 1.5
b = 10: ub = 20: i = 30: ui = 40: l = 50: ul = 60: q = 70: uq = 80: s = 90.5: d = 100.5: f = 110.4
PSet (b, 10), 15
PSet (ub, 10), 15
PSet (i, 10), 15
PSet (ui, 10), 15
PSet (l, 10), 15
PSet (ul, 10), 15
PSet (q, 10), 15
PSet (uq, 10), 15
PSet (s, 10), 15
PSet (d, 10), 15
PSet (f, 10), 15
PSet (120.5, 10.5), 15
PSet (121.5, 11.5), 15
PSet (2.5, 12), 15
PSet (3.5, 12), 15
Line (s, 20)-(d, 20.6)
Line (10, 30)-Step(d, .5), 14
b = 1: ub = 2: i = 3: ui = 4: l = 5: ul = 6: q = 4294967303: uq = 8: s = 8.5: d = 9.5: f = 10.5
PSet (10, 40), b
PSet (20, 40), ub
PSet (30, 40), i
PSet (40, 40), ui
PSet (50, 40), l
PSet (60, 40), ul
PSet (70, 40), q
PSet (80, 40), uq
PSet (90, 40), s
PSet (100, 40), d
PSet (110, 40), f
PSet (120, 40), 2.5
PSet (130, 40), 3.5
PSet (140, 40), 4294967301
Circle (250, 100), 10, 4294967302
Line (10, 50)-(60, 60), q, B , uq
Line (70, 50)-(120, 60), s, BF
Circle (40, 100), s, q
Circle (100, 100), i, d, s / 10, f / 2, ub / 4
Circle (160, 100), 20.5, 12.5
Paint (40, 100), d, q
Paint (100, 100), 2.5, 9.5
Color 13.5
s = 2.5: d = 3.5
Locate s, d
Print "row 2, column 4";
Locate 4.5, 5.5
Print "row 4, column 6";
q = 6: f = 2
Locate q, f
Print Point(s, d); Point(f); Point(q, 100)
PCopy 0.4, 1.4
Palette 3.5, 5.5
View (200.5, 150.5)-(250.5, 190.5), 2.5, 3.5
Window (q, f)-(s, d)
View Print 10.5 To 12.5
Print "viewport"
System
