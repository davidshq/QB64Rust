$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 7, PUT and GET of an erased whole array on a binary file,
' under a handler that prints ERR and resumes next.
ON ERROR GOTO h
REDIM a(2) AS LONG
a(0) = 1: a(1) = 2: a(2) = 3
OPEN "v62_c4.tmp" FOR BINARY AS #1
PUT #1, 1, a()
PRINT "1. LOF after PUT of a():"; LOF(1)
ERASE a
PUT #1, 13, a()
PRINT "2. LOF after PUT of the erased a():"; LOF(1)
GET #1, 1, a()
PRINT "3. after GET of the erased a()"
CLOSE #1
KILL "v62_c4.tmp"
PRINT "4. end"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
