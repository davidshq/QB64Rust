$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 7, SWAP of two erased whole arrays, one erased and one
' allocated, and INPUT # and LINE INPUT # into an erased whole array, under a handler that prints ERR and resumes
' next.
ON ERROR GOTO h
REDIM a(2) AS LONG, b(2) AS LONG, c(2) AS LONG
c(0) = 3
ERASE a, b
SWAP a(), b()
PRINT "1. after SWAP a(), b()"
SWAP a(), c()
PRINT "2. after SWAP a(), c():"; c(0)
OPEN "v62_c3.tmp" FOR OUTPUT AS #1
PRINT #1, "41"
PRINT #1, "text line"
CLOSE #1
OPEN "v62_c3.tmp" FOR INPUT AS #1
INPUT #1, a()
PRINT "3. after INPUT #1, a()"
REDIM l(2) AS STRING
ERASE l
LINE INPUT #1, l()
PRINT "4. after LINE INPUT #1, l()"
CLOSE #1
KILL "v62_c3.tmp"
PRINT "5. end"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
