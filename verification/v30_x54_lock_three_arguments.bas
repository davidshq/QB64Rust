$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (54_lock_three_arguments)
ON ERROR GOTO h
OPEN "v30_x54_lock_three_arguments.tmp" FOR BINARY AS #1
LOCK #1, 2, 4
CLOSE
KILL "v30_x54_lock_three_arguments.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
