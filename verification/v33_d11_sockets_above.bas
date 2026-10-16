$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): _SOCKETS_ and _DEBUG_ above a $DEBUG on the last line.
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$ELSE
    PRINT "sockets off"
$END IF
$IF _DEBUG_ THEN
    PRINT "debug on"
$ELSE
    PRINT "debug off"
$END IF
SYSTEM
$DEBUG
