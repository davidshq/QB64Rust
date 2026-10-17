$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a network handle never opened, -5: PRINT "statuscode -5:"; _STATUSCODE(-5)
ON ERROR GOTO handler
PRINT "statuscode -5:"; _STATUSCODE(-5)
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
