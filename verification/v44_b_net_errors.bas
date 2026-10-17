$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): the network functions where nothing listens and no
' handle is open: what each returns and the error it raises. No host is opened on a real port.
ON ERROR GOTO handler
PRINT "statuscode 0:"; _STATUSCODE(0)
PRINT "statuscode 20:"; _STATUSCODE(20)
PRINT "connected 0:"; _CONNECTED(0)
PRINT "address 0: ["; _CONNECTIONADDRESS$(0); "]"
PRINT "openconnection 0:"; _OPENCONNECTION(0)
PRINT "client nonsense:"; _OPENCLIENT("nonsense")
PRINT "client empty:"; _OPENCLIENT("")
PRINT "client lower case:"; _OPENCLIENT("tcp/ip:1:localhost")
PRINT "client no port:"; _OPENCLIENT("TCP/IP:x:localhost")
PRINT "client http bad name:"; _OPENCLIENT("HTTP:NotARealURL")
PRINT "client http ftp:"; _OPENCLIENT("HTTP:ftp://example.com")
PRINT "host nonsense:"; _OPENHOST("foobar")
PRINT "host no port:"; _OPENHOST("TCP/IP:abc")
PRINT "host empty:"; _OPENHOST("")
PRINT "constants:"; _CLIENT_FAILED; _HOST_FAILED; _CONNECTION_FAILED
s$ = "x"
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
