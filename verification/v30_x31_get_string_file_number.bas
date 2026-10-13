$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (31_get_string_file_number)
ON ERROR GOTO h
OPEN "v30_x31_get_string_file_number.tmp" FOR BINARY AS #1
DIM c AS STRING * 1
GET "f", , c
CLOSE
KILL "v30_x31_get_string_file_number.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
