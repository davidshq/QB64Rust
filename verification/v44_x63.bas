$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): a network handle never opened, -5: PRINT "eof -5:"; EOF(-5)
ON ERROR GOTO handler
PRINT "eof -5:"; EOF(-5)
PRINT "done"
SYSTEM

handler:
PRINT "  error"; ERR; "line"; _ERRORLINE
RESUME NEXT
