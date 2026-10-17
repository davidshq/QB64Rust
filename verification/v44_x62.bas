$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a network handle never opened, -5: PRINT "openconnection -5:"; _OPENCONNECTION(-5)
ON ERROR GOTO handler
PRINT "openconnection -5:"; _OPENCONNECTION(-5)
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
