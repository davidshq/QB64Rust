$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a network handle never opened, -5: PRINT "connected -5:"; _CONNECTED(-5)
ON ERROR GOTO handler
PRINT "connected -5:"; _CONNECTED(-5)
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
