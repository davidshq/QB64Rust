$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (16_using_file_no_values)
ON ERROR GOTO h
OPEN "v30_x16_using_file_no_values.tmp" FOR BINARY AS #1
PRINT #1, USING "##"
CLOSE
KILL "v30_x16_using_file_no_values.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
