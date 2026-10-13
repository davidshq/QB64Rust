$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (52_lock_to_nothing)
ON ERROR GOTO h
OPEN "v30_x52_lock_to_nothing.tmp" FOR BINARY AS #1
LOCK #1, 2 TO
CLOSE
KILL "v30_x52_lock_to_nothing.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
