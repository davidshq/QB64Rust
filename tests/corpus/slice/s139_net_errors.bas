$CONSOLE:ONLY
' The network functions where nothing listens and no handle is open (m3-network-embed; verification\v44_b_net_errors,
' v44_x60-x68): what each returns and the error it raises; EOF, LOF, GET, PUT and CLOSE of a handle never opened.
' No host is opened and nothing outside this machine is asked: a client to port 1 of localhost is refused, an
' HTTP name that is no host fails its lookup, ftp is not a protocol libqb's HTTP part allows. _STATUSCODE of a
' negative handle never opened ends the program (v44_x59) and is left out.
ON ERROR GOTO handler
PRINT "statuscode 0:"; _STATUSCODE(0)
PRINT "statuscode 20:"; _STATUSCODE(20)
PRINT "connected 0:"; _CONNECTED(0)
PRINT "address 0: ["; _CONNECTIONADDRESS$(0); "]"
PRINT "address without $: ["; _CONNECTIONADDRESS(0); "]"
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
h& = -5
PRINT "connected -5:"; _CONNECTED(h&)
PRINT "address -5: ["; _CONNECTIONADDRESS$(h&); "]"
PRINT "openconnection -5:"; _OPENCONNECTION(h&)
PRINT "eof -5:"; EOF(h&)
PRINT "lof -5:"; LOF(h&)
s$ = "x": GET #h&, , s$: PRINT "get -5: ["; s$; "]"
s$ = "y": PUT #h&, , s$: PRINT "put -5"
GET h&, , s$: PRINT "get without #: ["; s$; "]"
CLOSE #h&
PRINT "closed -5"
CLOSE h&, #-6
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
