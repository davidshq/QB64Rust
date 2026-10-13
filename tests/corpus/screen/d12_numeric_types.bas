' Arguments of every numeric type: a coordinate, a colour, a radius, a row and a screen mode, floats that round.
DIM b AS _BYTE, ub AS _UNSIGNED _BYTE, i AS INTEGER, ui AS _UNSIGNED INTEGER, l AS LONG, ul AS _UNSIGNED LONG
DIM q AS _INTEGER64, uq AS _UNSIGNED _INTEGER64, s AS SINGLE, d AS DOUBLE, f AS _FLOAT
s = 7.4
SCREEN s
CLS 1.4, 1.5
b = 10: ub = 20: i = 30: ui = 40: l = 50: ul = 60: q = 70: uq = 80: s = 90.5: d = 100.5: f = 110.4
PSET (b, 10), 15
PSET (ub, 10), 15
PSET (i, 10), 15
PSET (ui, 10), 15
PSET (l, 10), 15
PSET (ul, 10), 15
PSET (q, 10), 15
PSET (uq, 10), 15
PSET (s, 10), 15
PSET (d, 10), 15
PSET (f, 10), 15
PSET (120.5, 10.5), 15
PSET (121.5, 11.5), 15
PSET (2.5, 12), 15
PSET (3.5, 12), 15
LINE (s, 20)-(d, 20.6)
LINE (10, 30)-STEP(d, .5), 14
b = 1: ub = 2: i = 3: ui = 4: l = 5: ul = 6: q = 4294967303: uq = 8: s = 8.5: d = 9.5: f = 10.5
PSET (10, 40), b
PSET (20, 40), ub
PSET (30, 40), i
PSET (40, 40), ui
PSET (50, 40), l
PSET (60, 40), ul
PSET (70, 40), q
PSET (80, 40), uq
PSET (90, 40), s
PSET (100, 40), d
PSET (110, 40), f
PSET (120, 40), 2.5
PSET (130, 40), 3.5
PSET (140, 40), 4294967301
CIRCLE (250, 100), 10, 4294967302
LINE (10, 50)-(60, 60), q, B, uq
LINE (70, 50)-(120, 60), s, BF
CIRCLE (40, 100), s, q
CIRCLE (100, 100), i, d, s / 10, f / 2, ub / 4
CIRCLE (160, 100), 20.5, 12.5
PAINT (40, 100), d, q
PAINT (100, 100), 2.5, 9.5
COLOR 13.5
s = 2.5: d = 3.5
LOCATE s, d
PRINT "row 2, column 4";
LOCATE 4.5, 5.5
PRINT "row 4, column 6";
q = 6: f = 2
LOCATE q, f
PRINT POINT(s, d); POINT(f); POINT(q, 100)
PCOPY 0.4, 1.4
PALETTE 3.5, 5.5
VIEW (200.5, 150.5)-(250.5, 190.5), 2.5, 3.5
WINDOW (q, f)-(s, d)
VIEW PRINT 10.5 TO 12.5
PRINT "viewport"
SYSTEM
