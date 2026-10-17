$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a network handle never opened, -5: PRINT "address -5: ["; _CONNECTIONADDRESS$(-5); "]"
ON ERROR GOTO handler
PRINT "address -5: ["; _CONNECTIONADDRESS$(-5); "]"
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
