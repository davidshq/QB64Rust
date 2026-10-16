$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (56_call_lock)
ON ERROR GOTO h
OPEN "v30_x56_call_lock.tmp" FOR BINARY AS #1
CALL LOCK(1, 2 TO 3)
CALL UNLOCK(#1, 2 TO 3)
CLOSE
KILL "v30_x56_call_lock.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
