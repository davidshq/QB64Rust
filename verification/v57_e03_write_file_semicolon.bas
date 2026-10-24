$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): WRITE #1; with a ; after the file number.
OPEN "v57_e03.tmp" FOR OUTPUT AS #1
WRITE #1; "a"
CLOSE #1
KILL "v57_e03.tmp"
PRINT "ok"
SYSTEM
