$CONSOLE:ONLY
OPEN "reset_1.tmp" FOR OUTPUT AS #1
OPEN "reset_1b.tmp" FOR OUTPUT AS #2
RESET
PRINT "After RESET, FREEFILE is"; FREEFILE
KILL "reset_1.tmp": KILL "reset_1b.tmp"
END
