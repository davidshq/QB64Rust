$CONSOLE:ONLY
OPEN "kill_1.tmp" FOR OUTPUT AS #1: CLOSE #1
PRINT "Before KILL:"; _FILEEXISTS("kill_1.tmp")
KILL "kill_1.tmp"
PRINT "After KILL:"; _FILEEXISTS("kill_1.tmp")
ON ERROR GOTO handler
KILL "kill_1.tmp"
END

handler:
PRINT "KILL of a missing file: error"; ERR
RESUME NEXT
