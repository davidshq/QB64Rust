' PRINT and WRITE in a window: each separator, zones of 14 columns, a line that wraps, enough lines to scroll.
DIM i AS INTEGER, s AS STRING
FOR i = 1 TO 27
    PRINT "line"; i
NEXT i
PRINT "plain"
PRINT 1; -2; 3.5; "text"; 4
PRINT "a", "b", 12, "d"
PRINT "no line end";
PRINT " goes on"
PRINT
PRINT "comma at the end",
PRINT "next zone"
PRINT "1234567890123456789012345678", "past two zones"
WRITE 1, "two", 3.5
s = STRING$(100, "w")
PRINT s
PRINT "0123456789012345678901234567890123456789012345678901234567890123456789012345"; "wraps whole"
PRINT "last";
SYSTEM
