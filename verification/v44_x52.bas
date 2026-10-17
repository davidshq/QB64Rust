$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _SOCKETS_ with _OPENHOST in a SUB never called, after the $IF
$IF _SOCKETS_ THEN
PRINT "sockets"
$ELSE
PRINT "no sockets"
$END IF
SYSTEM
SUB s
PRINT _OPENHOST("x")
END SUB
