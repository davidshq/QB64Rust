$CONSOLE:ONLY
$DEBUG
' Verification (m3-asserts-debug-zlib, task 1.1): what $DEBUG changes outside the IDE: flags, ERL, an error in a SUB, an untrapped error.
DIM SHARED a(10)
PRINT "start"
t! = TIMER
$IF _DEBUG_ THEN
    PRINT "debug on"
$ELSE
    PRINT "debug off"
$END IF
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$ELSE
    PRINT "sockets off"
$END IF
ON ERROR GOTO handler
10 x = 1
20 ERROR 5
30 PRINT "after error"; ERL
s
PRINT "fast"; TIMER - t! < 1
PRINT "done"
a(11) = 1
PRINT "not reached"
SYSTEM
handler:
PRINT "handler"; ERR; ERL; _ERRORLINE
RESUME NEXT
SUB s
    ERROR 6
    PRINT "in s"; ERR
END SUB
