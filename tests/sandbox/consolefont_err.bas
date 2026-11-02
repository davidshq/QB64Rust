' _CONSOLEFONT sets the font of the program's own console, which nothing in BASIC reads back: ERR after it, and that
' the program goes on (tests/sandbox/README.md). The output goes to out.txt: the console must be a real one.
$CONSOLE:ONLY
OPEN "out.txt" FOR OUTPUT AS #1
ON ERROR GOTO failed

PRINT "before"
_CONSOLEFONT "Consolas", 20
PRINT #1, "ERR after _CONSOLEFONT:"; e
_CONSOLEFONT "", 12
PRINT #1, "ERR after the size alone:"; e
PRINT "after"
PRINT #1, "went on"
CLOSE #1
SYSTEM

failed:
e = ERR
RESUME NEXT
