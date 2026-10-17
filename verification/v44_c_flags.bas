$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _SOCKETS_ is 1 for the whole program when _OPENCLIENT
' is used anywhere in its compiled code, here in a SUB that is never called, below the $IF that reads it.
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$ELSE
    PRINT "sockets off"
$END IF
$IF _SOCKETS_ = 1 THEN
    PRINT "sockets = 1"
$END IF
PRINT "url:"; _ENCODEURL$("a b"); " "; _DECODEURL$("a%20b")
SYSTEM

SUB never
    PRINT _OPENCLIENT("TCP/IP:1:localhost")
END SUB
