$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (41_call_get)
ON ERROR GOTO h
OPEN "v30_x41_call_get.tmp" FOR BINARY AS #1
DIM c AS STRING * 1
CALL GET(1, 1, c)
PRINT SEEK(1)
CLOSE
KILL "v30_x41_call_get.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
