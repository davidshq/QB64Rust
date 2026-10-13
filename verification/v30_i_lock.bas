$CONSOLE:ONLY
' Verification (step 4 of M3, measured before its design): LOCK and UNLOCK. Each form the template allows, on a
' file open in each mode, a file that is not open, an UNLOCK with no LOCK before it, a second LOCK, a second OPEN
' of a locked file and a read through it, and whether a LOCK holds after CLOSE. The file is v30_i.tmp, removed.
ON ERROR GOTO h
DIM f AS STRING, c AS STRING * 1, n AS LONG
f = "v30_i.tmp"
OPEN f FOR OUTPUT AS #1: PRINT #1, "abcdefghij": CLOSE #1
PRINT "forms, BINARY"
OPEN f FOR BINARY AS #1
PRINT "whole:";: LOCK #1: UNLOCK #1: PRINT
PRINT "no #:";: LOCK 1: UNLOCK 1: PRINT
PRINT "one record:";: LOCK #1, 2: UNLOCK #1, 2: PRINT
PRINT "a range:";: LOCK #1, 2 TO 4: UNLOCK #1, 2 TO 4: PRINT
PRINT "a comma alone:";: LOCK #1, : UNLOCK #1, : PRINT
n = 1: PRINT "a variable:";: LOCK #n, n TO n + 2: UNLOCK #n, n TO n + 2: PRINT
PRINT "record 0:";: LOCK #1, 0: UNLOCK #1, 0: PRINT
PRINT "record -1:";: LOCK #1, -1: PRINT
PRINT "a range the wrong way round:";: LOCK #1, 4 TO 2: UNLOCK #1, 4 TO 2: PRINT
PRINT "past the end:";: LOCK #1, 100 TO 200: UNLOCK #1, 100 TO 200: PRINT
PRINT "UNLOCK with no LOCK:";: UNLOCK #1: PRINT
PRINT "UNLOCK of another range:";: LOCK #1, 2 TO 4: UNLOCK #1, 2 TO 3: PRINT : UNLOCK #1, 2 TO 4
PRINT "LOCK twice:";: LOCK #1: LOCK #1: PRINT : UNLOCK #1: UNLOCK #1
PRINT "read and write while locked:";: LOCK #1: GET #1, 1, c: PRINT c;: PUT #1, 1, c: UNLOCK #1: PRINT
PRINT "a second OPEN while locked:";: LOCK #1
OPEN f FOR BINARY AS #2: c = "?": GET #2, 1, c: PRINT c;: CLOSE #2: UNLOCK #1: PRINT
PRINT "CLOSE while locked:";: LOCK #1: CLOSE #1: PRINT
OPEN f FOR BINARY AS #1: c = "?": GET #1, 1, c: PRINT c: CLOSE #1
PRINT "other modes"
OPEN f FOR RANDOM AS #1 LEN = 2
PRINT "RANDOM:";: LOCK #1: UNLOCK #1: LOCK #1, 2: UNLOCK #1, 2: LOCK #1, 2 TO 3: UNLOCK #1, 2 TO 3: PRINT
CLOSE #1
OPEN f FOR INPUT AS #1
PRINT "INPUT:";: LOCK #1: UNLOCK #1: LOCK #1, 2: UNLOCK #1, 2: PRINT
CLOSE #1
OPEN f FOR APPEND AS #1
PRINT "APPEND:";: LOCK #1: UNLOCK #1: PRINT
CLOSE #1
OPEN f FOR OUTPUT AS #1
PRINT "OUTPUT:";: LOCK #1: PRINT #1, "x": UNLOCK #1: PRINT
CLOSE #1
PRINT "not open:";: LOCK #3: UNLOCK #3: PRINT
PRINT "number 0:";: LOCK #0: PRINT
KILL f
SYSTEM
h:
PRINT "<error"; ERR; ">";
RESUME NEXT
