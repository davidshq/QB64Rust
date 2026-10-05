$IF _WIN THEN
    PRINT "Windows"
$ELSEIF _MAC THEN
    PRINT "macOS"
$ELSEIF _LINUX THEN
    PRINT "Linux"
$ELSE
    PRINT "Unknown"
$END IF
