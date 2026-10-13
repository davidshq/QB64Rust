$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (32_get_string_position)
ON ERROR GOTO h
OPEN "v30_x32_get_string_position.tmp" FOR BINARY AS #1
DIM c AS STRING * 1
GET #1, "a", c
CLOSE
KILL "v30_x32_get_string_position.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
