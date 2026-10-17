$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a network handle never opened, -5: GET #-5, , s$
ON ERROR GOTO handler
s$ = "x": GET #-5, , s$: PRINT "get -5: ["; s$; "]"
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
