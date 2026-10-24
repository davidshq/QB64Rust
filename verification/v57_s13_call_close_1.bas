$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): the stub CALL CLOSE(1).
OPEN "v57_s13.tmp" FOR OUTPUT AS #1
CALL CLOSE(1)
PRINT "open:"; FREEFILE
KILL "v57_s13.tmp"
PRINT "ok"
SYSTEM
