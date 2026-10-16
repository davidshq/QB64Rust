$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (38_put_whole_string_array)
ON ERROR GOTO h
OPEN "v30_x38_put_whole_string_array.tmp" FOR BINARY AS #1
DIM sa(1 TO 2) AS STRING
sa(1) = "ab": sa(2) = "cde"
PUT #1, 1, sa()
PRINT LOF(1)
CLOSE
KILL "v30_x38_put_whole_string_array.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
