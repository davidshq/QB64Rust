DIM x AS INTEGER
$IF _LINUX THEN
    x = 1
    PRINT "Linux"
    x = x + 1
$ELSE
    x = 9999
    PRINT "Other"
$END IF
