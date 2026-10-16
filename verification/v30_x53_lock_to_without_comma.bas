$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (53_lock_to_without_comma)
ON ERROR GOTO h
OPEN "v30_x53_lock_to_without_comma.tmp" FOR BINARY AS #1
LOCK #1 TO 4
CLOSE
KILL "v30_x53_lock_to_without_comma.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
