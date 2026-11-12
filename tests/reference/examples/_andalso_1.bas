$CONSOLE:ONLY
DIM a(1 TO 3) AS INTEGER
a(1) = 10: a(2) = 20: a(3) = 30
i = 4
IF i <= 3 _ANDALSO a(i) > 15 THEN PRINT "Found." ELSE PRINT "Index"; i; "is out of range; a(i) was not read."
PRINT "2 _ANDALSO 4 is"; 2 _ANDALSO 4; "while 2 AND 4 is"; 2 AND 4
END
