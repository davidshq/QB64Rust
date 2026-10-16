' COLOR and CLS in graphics modes: the colour drawing statements take when none is given, CLS with a viewport.
SCREEN 7
LINE (300, 180)-(319, 199), 5, BF
CLS , 4
COLOR 3
PRINT "cyan"
LINE (0, 20)-(50, 30)
COLOR 14, 1
PRINT "yellow, background 1"
PSET (60, 25)
VIEW (100, 50)-(200, 150), 2, 15
LINE (10, 10)-(40, 40), 4, BF
CLS 1
PSET (5, 5)
VIEW PRINT 10 TO 20
PRINT "text viewport"
CLS 2
PRINT "after CLS 2"
VIEW PRINT
VIEW (220, 100)-(300, 180), 5, 15
CLS
VIEW
LINE (250, 150)-(310, 190), , B
SYSTEM
