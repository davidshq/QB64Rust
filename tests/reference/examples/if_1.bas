$CONSOLE:ONLY
score = 72
IF score >= 90 THEN
    PRINT "Grade A"
ELSEIF score >= 70 THEN
    PRINT "Grade B"
ELSE
    PRINT "Grade C"
END IF
IF score > 50 THEN PRINT "Passed." ELSE PRINT "Failed."
lives = 2
IF lives THEN PRINT "Any value but 0 is true:"; lives
END
