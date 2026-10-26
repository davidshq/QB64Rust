$Console:Only
' A TCP/IP exchange over the loopback address (m3-network-embed; verification\v44_e_loopback): a host on a fixed
' port, a client to localhost, the connection, a string each way with PUT and GET, _CONNECTED, the client's and the
' connection's addresses (the connection's port is the client's, chosen by the system, so it is not printed),
' CLOSE. Never a host's own address, which asks the internet. The program listens on port 47392: Windows may ask
' once whether it may; loopback works either way.
h& = _OpenHost("TCP/IP:47392")
Print "host:"; h& < 0
c& = _OpenClient("TCP/IP:47392:localhost")
Print "client:"; c& < 0; c& <> h&
t# = Timer
Do
    cn& = _OpenConnection(h&)
    If cn& <> 0 Or Timer - t# > 5 Then Exit Do
    _Limit 100
Loop
Print "connection:"; cn& < 0; cn& <> c&; cn& <> h&
Print "connected:"; _Connected(c&); _Connected(cn&)
Print "client address: "; _ConnectionAddress$(c&)
a$ = _ConnectionAddress(cn&)
p1% = InStr(a$, ":"): p2% = InStr(p1% + 1, a$, ":")
Print "connection address: "; Left$(a$, p1%); "<port>"; Mid$(a$, p2%); Val(Mid$(a$, p1% + 1)) > 0
s$ = "hello from the client"
Put #c&, , s$
r$ = "": t# = Timer
Do
    Get #cn&, , g$
    r$ = r$ + g$
    If Len(r$) >= Len(s$) Or Timer - t# > 5 Then Exit Do
    _Limit 100
Loop
Print "host got: ["; r$; "]"
s$ = "and back" + Chr$(0) + Chr$(255) + "!"
Put #cn&, , s$
r$ = "": t# = Timer
Do
    Get #c&, , g$
    r$ = r$ + g$
    If Len(r$) >= Len(s$) Or Timer - t# > 5 Then Exit Do
    _Limit 100
Loop
Print "client got:"; Len(r$); r$ = s$; Asc(r$, 10)
Print "eof:"; EOF(c&); "lof:"; LOF(c&)
Close #c&
t# = Timer
Do
    Get #cn&, , g$
    If _Connected(cn&) = 0 Or Timer - t# > 5 Then Exit Do
    _Limit 100
Loop
Print "after the client closed:"; _Connected(cn&)
Close #cn&
Close #h&
Print "done"
System
