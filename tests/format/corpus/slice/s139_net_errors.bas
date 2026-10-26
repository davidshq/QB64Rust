$Console:Only
' The network functions where nothing listens and no handle is open (m3-network-embed; verification\v44_b_net_errors,
' v44_x60-x68): what each returns and the error it raises; EOF, LOF, GET, PUT and CLOSE of a handle never opened.
' No host is opened and nothing outside this machine is asked: a client to port 1 of localhost is refused, an
' HTTP name that is no host fails its lookup, ftp is not a protocol libqb's HTTP part allows. _STATUSCODE of a
' negative handle never opened ends the program (v44_x59) and is left out.
On Error GoTo handler
Print "statuscode 0:"; _StatusCode(0)
Print "statuscode 20:"; _StatusCode(20)
Print "connected 0:"; _Connected(0)
Print "address 0: ["; _ConnectionAddress$(0); "]"
Print "address without $: ["; _ConnectionAddress(0); "]"
Print "openconnection 0:"; _OpenConnection(0)
Print "client nonsense:"; _OpenClient("nonsense")
Print "client empty:"; _OpenClient("")
Print "client lower case:"; _OpenClient("tcp/ip:1:localhost")
Print "client no port:"; _OpenClient("TCP/IP:x:localhost")
Print "client http bad name:"; _OpenClient("HTTP:NotARealURL")
Print "client http ftp:"; _OpenClient("HTTP:ftp://example.com")
Print "host nonsense:"; _OpenHost("foobar")
Print "host no port:"; _OpenHost("TCP/IP:abc")
Print "host empty:"; _OpenHost("")
h& = -5
Print "connected -5:"; _Connected(h&)
Print "address -5: ["; _ConnectionAddress$(h&); "]"
Print "openconnection -5:"; _OpenConnection(h&)
Print "eof -5:"; EOF(h&)
Print "lof -5:"; LOF(h&)
s$ = "x": Get #h&, , s$: Print "get -5: ["; s$; "]"
s$ = "y": Put #h&, , s$: Print "put -5"
Get h&, , s$: Print "get without #: ["; s$; "]"
Close #h&
Print "closed -5"
Close h&, #-6
Print "done"
System

handler:
Print "  error"; Err; "line"; _ErrorLine
Resume Next
