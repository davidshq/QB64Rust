' TEST: typed
' The network functions (m3-network-embed design D5): _OPENHOST, _OPENCLIENT, _OPENCONNECTION, _CONNECTED and
' _STATUSCODE LONG, _CONNECTIONADDRESS a string with or without its $; a handle a LONG slot (a float rounded).
$CONSOLE:ONLY
DIM s AS STRING, f AS STRING * 4, k AS LONG, d AS DOUBLE, q AS _INTEGER64
k = _OPENHOST("TCP/IP:" + s)
k = _OPENCLIENT(f)
k = _OPENCONNECTION(d)
k = _CONNECTED(q)
k = _STATUSCODE(k)
s = _CONNECTIONADDRESS$(k)
s = _CONNECTIONADDRESS(d)
SYSTEM
