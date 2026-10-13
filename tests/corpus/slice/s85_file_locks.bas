$CONSOLE:ONLY
' Slice (m3-console-files, task 5.3; from verification\v30_i_lock): LOCK and UNLOCK, 11 numbered lines. The three
' forms with and without the number sign, a comma alone, variables; record 0 and below (error 5), a range the wrong
' way round (75), an UNLOCK that matches no LOCK and a second LOCK of the same bytes (70); the program reads and
' writes what it locked; a second OPEN of the file inside the program cannot read it (70); CLOSE ends the lock;
' every mode takes it; a file that is not open (52); after CALL. One program, one process: nothing here needs a
' second program. The file is s85_file_locks.tmp and is removed.
ON ERROR GOTO handler
DIM f AS STRING, c AS STRING * 1, n AS LONG
f = "s85_file_locks.tmp"
OPEN f FOR OUTPUT AS #1: PRINT #1, "abcdefghij": CLOSE #1
OPEN f FOR BINARY AS #1
PRINT "1:";: LOCK #1: UNLOCK #1: LOCK 1: UNLOCK 1: LOCK #1, 2: UNLOCK #1, 2: LOCK #1, 2 TO 4: UNLOCK #1, 2 TO 4: PRINT " forms"
n = 1
PRINT "2:";: LOCK #1,: UNLOCK #1,: LOCK #n, n TO n + 2: UNLOCK #n, n TO n + 2: CALL LOCK(1, 2 TO 3): CALL UNLOCK(#1, 2 TO 3): PRINT " more forms"
PRINT "3:";: LOCK #1, 0: UNLOCK #1, 0: LOCK #1, -1: PRINT
PRINT "4:";: LOCK #1, 4 TO 2: UNLOCK #1, 4 TO 2: PRINT
PRINT "5:";: LOCK #1, 100 TO 200: UNLOCK #1, 100 TO 200: PRINT " past the end"
PRINT "6:";: UNLOCK #1: LOCK #1, 2 TO 4: UNLOCK #1, 2 TO 3: UNLOCK #1, 2 TO 4: PRINT
PRINT "7:";: LOCK #1: LOCK #1: UNLOCK #1: UNLOCK #1: PRINT
PRINT "8:";: LOCK #1: GET #1, 1, c: PRINT " "; c;: PUT #1, 1, c: UNLOCK #1: PRINT
PRINT "9:";: LOCK #1
OPEN f FOR BINARY AS #2: c = "?": GET #2, 1, c: PRINT " "; c;: CLOSE #2: UNLOCK #1
LOCK #1: CLOSE #1
OPEN f FOR BINARY AS #1: c = "?": GET #1, 1, c: PRINT " "; c: CLOSE #1
OPEN f FOR RANDOM AS #1 LEN = 2
PRINT "10:";: LOCK #1: UNLOCK #1: LOCK #1, 2: UNLOCK #1, 2: LOCK #1, 2 TO 3: UNLOCK #1, 2 TO 3: CLOSE #1
OPEN f FOR INPUT AS #1: LOCK #1: UNLOCK #1: LOCK #1, 2: UNLOCK #1, 2: CLOSE #1
OPEN f FOR APPEND AS #1: LOCK #1: UNLOCK #1: CLOSE #1
OPEN f FOR OUTPUT AS #1: LOCK #1: PRINT #1, "x": UNLOCK #1: CLOSE #1
PRINT " every mode"
PRINT "11:";: LOCK #3: UNLOCK #3: LOCK #0: PRINT
KILL f
SYSTEM
handler:
PRINT " <error"; ERR; ">";
RESUME NEXT
