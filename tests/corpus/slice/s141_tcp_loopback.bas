$CONSOLE:ONLY
' A TCP/IP exchange over the loopback address (m3-network-embed; verification\v44_e_loopback): a host on a fixed
' port, a client to localhost, the connection, a string each way with PUT and GET, _CONNECTED, the client's and the
' connection's addresses (the connection's port is the client's, chosen by the system, so it is not printed),
' CLOSE. Never a host's own address, which asks the internet. The program listens on port 47392: Windows may ask
' once whether it may; loopback works either way.
h& = _OPENHOST("TCP/IP:47392")
PRINT "host:"; h& < 0
c& = _OPENCLIENT("TCP/IP:47392:localhost")
PRINT "client:"; c& < 0; c& <> h&
t# = TIMER
DO
    cn& = _OPENCONNECTION(h&)
    IF cn& <> 0 OR TIMER - t# > 5 THEN EXIT DO
    _LIMIT 100
LOOP
PRINT "connection:"; cn& < 0; cn& <> c&; cn& <> h&
PRINT "connected:"; _CONNECTED(c&); _CONNECTED(cn&)
PRINT "client address: "; _CONNECTIONADDRESS$(c&)
a$ = _CONNECTIONADDRESS(cn&)
p1% = INSTR(a$, ":"): p2% = INSTR(p1% + 1, a$, ":")
PRINT "connection address: "; LEFT$(a$, p1%); "<port>"; MID$(a$, p2%); VAL(MID$(a$, p1% + 1)) > 0
s$ = "hello from the client"
PUT #c&, , s$
r$ = "": t# = TIMER
DO
    GET #cn&, , g$
    r$ = r$ + g$
    IF LEN(r$) >= LEN(s$) OR TIMER - t# > 5 THEN EXIT DO
    _LIMIT 100
LOOP
PRINT "host got: ["; r$; "]"
s$ = "and back" + CHR$(0) + CHR$(255) + "!"
PUT #cn&, , s$
r$ = "": t# = TIMER
DO
    GET #c&, , g$
    r$ = r$ + g$
    IF LEN(r$) >= LEN(s$) OR TIMER - t# > 5 THEN EXIT DO
    _LIMIT 100
LOOP
PRINT "client got:"; LEN(r$); r$ = s$; ASC(r$, 10)
PRINT "eof:"; EOF(c&); "lof:"; LOF(c&)
CLOSE #c&
t# = TIMER
DO
    GET #cn&, , g$
    IF _CONNECTED(cn&) = 0 OR TIMER - t# > 5 THEN EXIT DO
    _LIMIT 100
LOOP
PRINT "after the client closed:"; _CONNECTED(cn&)
CLOSE #cn&
CLOSE #h&
PRINT "done"
SYSTEM
