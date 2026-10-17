$CONSOLE:ONLY
' Verification (m3-network-embed, by its review): the same file with another separator after ./
$EMBED:'./v44_files/text.txt','h'
$EMBED:'./v44_files\text.txt','g'
PRINT "accepted:"; LEN(_EMBEDDED$("h")); LEN(_EMBEDDED$("g"))
SYSTEM
