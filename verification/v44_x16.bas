$CONSOLE:ONLY
' Verification (m3-network-embed, before its proposal): $EMBED in an included file, a ./ name
'$INCLUDE:'v44_inc/embed.bi'
PRINT "accepted:"; LEN(_EMBEDDED$("h"))
SYSTEM
