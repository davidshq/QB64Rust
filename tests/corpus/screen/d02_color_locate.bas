' COLOR and LOCATE in SCREEN 0, with arguments left out, and the shape of the cursor.
COLOR 14, 1
PRINT "yellow on blue"
COLOR , 2
PRINT "background only"
COLOR 12
PRINT "foreground only"
COLOR 7, 0, 4
PRINT "with a border colour"
COLOR 31, 0
PRINT "blinking"
COLOR 7
LOCATE 10, 20
PRINT "at 10, 20"
LOCATE , 40
PRINT "column 40";
LOCATE 12
PRINT "row 12";
LOCATE 14, 5, 0
PRINT "cursor off";
LOCATE , , 1
LOCATE 16, 5, 1, 3, 5
PRINT "cursor shape";
LOCATE , , , 1, 7
LOCATE 20, 30
SYSTEM
