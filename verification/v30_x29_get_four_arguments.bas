$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (29_get_four_arguments)
ON ERROR GOTO h
OPEN "v30_x29_get_four_arguments.tmp" FOR BINARY AS #1
DIM c AS STRING * 1
GET #1, 1, c, 2
CLOSE
KILL "v30_x29_get_four_arguments.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
