$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (43_put_whole_fixed_string_array)
ON ERROR GOTO h
OPEN "v30_x43_put_whole_fixed_string_array.tmp" FOR BINARY AS #1
DIM fa(1 TO 2) AS STRING * 3
fa(1) = "ab": fa(2) = "cde"
PUT #1, 1, fa()
PRINT LOF(1)
CLOSE
KILL "v30_x43_put_whole_fixed_string_array.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
