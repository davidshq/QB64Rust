' CLS with its arguments and VIEW PRINT in SCREEN 0: a text viewport scrolls and is cleared alone.
DIM i AS INTEGER
COLOR 15, 4
CLS
PRINT "white on red after CLS"
CLS , 2
PRINT "after CLS with a background"
FOR i = 3 TO 20
    LOCATE i, 1
    PRINT "row"; i;
NEXT i
VIEW PRINT 5 TO 10
PRINT "in the viewport"
FOR i = 1 TO 9
    PRINT "scrolls"; i
NEXT i
VIEW PRINT 12 TO 14
CLS 2
PRINT "cleared by CLS 2"
VIEW PRINT 16 TO 18
CLS
PRINT "cleared by CLS"
VIEW PRINT
LOCATE 22, 1
PRINT "the whole screen again";
COLOR 7, 0
SYSTEM
