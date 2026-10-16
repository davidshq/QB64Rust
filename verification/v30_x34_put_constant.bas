$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (34_put_constant)
ON ERROR GOTO h
OPEN "v30_x34_put_constant.tmp" FOR BINARY AS #1
CONST k = 5
PUT #1, , k
CLOSE
KILL "v30_x34_put_constant.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
