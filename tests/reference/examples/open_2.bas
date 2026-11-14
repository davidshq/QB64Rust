$CONSOLE:ONLY
' The run-time errors of OPEN, each caught by the handler, which prints its number.
ON ERROR GOTO failed
step$ = "a file that is not there, for input": OPEN "open_2.none" FOR INPUT AS #1
step$ = "a folder that is not there": OPEN "open_2_nodir/x.txt" FOR OUTPUT AS #1
step$ = "an empty name": OPEN "" FOR OUTPUT AS #1
step$ = "a file number of 0": OPEN "open_2.tmp" FOR OUTPUT AS #0
step$ = "a file number already open": OPEN "open_2.tmp" FOR OUTPUT AS #1: OPEN "open_2.tmp" FOR OUTPUT AS #1
CLOSE #1
step$ = "a mode letter that is no mode": OPEN "Q", #1, "open_2.tmp"
step$ = "a folder as the file": OPEN "." FOR INPUT AS #1
CLOSE
KILL "open_2.tmp"
END

failed:
PRINT step$; ": error"; ERR
RESUME NEXT
