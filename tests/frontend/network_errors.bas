' TEST: check-fail
' The forms of the network functions the old compiler rejects (m3-network-embed design D5; verification/v44_x40-x45,
' x56, x57): no argument or two, a number for a string, a string for a number, a function as a statement, and the
' same for the URL functions. Each is an error of the program, none "not supported yet"; the messages are new. A
' suffix (x46, "Invalid array name") stays "not supported yet", as for every name with a leading underscore and
' parentheses that names no array (v33_x25-x29).
$CONSOLE:ONLY
PRINT _OPENCLIENT
PRINT _OPENCLIENT("a", "b")
PRINT _OPENCLIENT(5)
_OPENHOST "TCP/IP:1"
PRINT _STATUSCODE("x")
PRINT _OPENCONNECTION()
PRINT _CONNECTED(1, 2)
PRINT _CONNECTIONADDRESS$("x")
PRINT _OPENHOST(1)
PRINT _DECODEURL$(5)
_ENCODEURL$ "a b"
SYSTEM
