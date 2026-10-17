$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): _CONNECTIONADDRESS without the $
ON ERROR GOTO h
PRINT "["; _CONNECTIONADDRESS(0); "]"
SYSTEM
h: PRINT "error"; ERR: RESUME NEXT
