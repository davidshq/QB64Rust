' TEST: ir
' _SOCKETS_ is 1 for the whole program when _OPENCLIENT stands in a SUB that is never called, below the $IF that
' reads it (m3-network-embed design D6; verification/v44_c_flags): the branch is taken, and the program links the
' sockets part.
$CONSOLE:ONLY
$IF _SOCKETS_ THEN
    PRINT "sockets on"
$ELSE
    PRINT "sockets off"
$END IF
SYSTEM

SUB never
    PRINT _OPENCLIENT("TCP/IP:1:localhost")
END SUB
