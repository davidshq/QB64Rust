$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _SOCKETS_ with _OPENCONNECTION and _CONNECTED only
$IF _SOCKETS_ THEN
PRINT "sockets"
$ELSE
PRINT "no sockets"
$END IF
IF 0 THEN PRINT _OPENCONNECTION(0); _CONNECTED(0); _STATUSCODE(0)
SYSTEM
