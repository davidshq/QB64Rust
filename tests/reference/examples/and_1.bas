$CONSOLE:ONLY
flags = 6
PRINT "6 AND 4 is"; flags AND 4; "and 6 AND 1 is"; flags AND 1
age = 30
IF age >= 18 AND age < 65 THEN PRINT "Of working age."
PRINT "A true comparison is"; (age > 18); "and AND keeps it:"; (age > 18) AND (age < 65)
END
