$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _SOCKETS_ with _OPENCLIENT only in a $IF branch not taken
$IF _SOCKETS_ THEN
PRINT "sockets"
$ELSE
PRINT "no sockets"
$END IF
$IF 0 THEN
PRINT _OPENCLIENT("x")
$END IF
SYSTEM
