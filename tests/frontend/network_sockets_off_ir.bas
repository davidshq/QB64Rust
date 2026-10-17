' TEST: ir
' _SOCKETS_ stays 0 for _OPENCLIENT in a $IF branch not taken and for the other four network functions
' (m3-network-embed design D6; verification/v44_x51, x53): the $ELSE branch is taken, and no sockets part is linked.
$CONSOLE:ONLY
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$ELSE
    PRINT "sockets off"
$END IF
$IF 0 THEN
    PRINT _OPENCLIENT("x")
$END IF
IF 0 THEN PRINT _OPENCONNECTION(0); _CONNECTED(0); _STATUSCODE(0); _CONNECTIONADDRESS$(0)
SYSTEM
