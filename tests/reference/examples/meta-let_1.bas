$CONSOLE:ONLY
$LET DEBUG = 1
$IF DEBUG THEN
    PRINT "DEBUG is 1: the debug lines are compiled."
$END IF
$LET DEBUG = 0
$IF DEBUG THEN
    PRINT "This line is left out."
$ELSE
    PRINT "DEBUG is now 0."
$END IF
$LET MODE = fast
$IF MODE = FAST THEN
    PRINT "MODE is FAST: the letter case of a value does not count."
$END IF
END
