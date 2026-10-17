$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a network handle never opened, -5: PUT #-5, , s$
ON ERROR GOTO handler
s$ = "x": PUT #-5, , s$
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
