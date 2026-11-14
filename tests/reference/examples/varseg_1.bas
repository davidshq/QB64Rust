$CONSOLE:ONLY
DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER
PRINT "Paragraphs from a to b:"; VARSEG(b(1)) - VARSEG(a(1))
PRINT "One segment for the whole array:"; VARSEG(a(1)) = VARSEG(a(7))
PRINT "Segment of a variable:"; VARSEG(n%)
END
