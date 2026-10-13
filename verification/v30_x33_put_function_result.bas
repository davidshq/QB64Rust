$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (33_put_function_result)
ON ERROR GOTO h
OPEN "v30_x33_put_function_result.tmp" FOR BINARY AS #1
DIM v AS STRING
v = "ab"
PUT #1, , MID$(v, 1, 1)
CLOSE
KILL "v30_x33_put_function_result.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
