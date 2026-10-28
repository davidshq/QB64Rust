$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 7, FIELD, LSET and RSET of an erased whole STRING array,
' under a handler that prints ERR and resumes next.
ON ERROR GOTO h
REDIM e(2) AS STRING
ERASE e
LSET e() = "ab"
PRINT "1. after LSET e()"
RSET e() = "cd"
PRINT "2. after RSET e()"
OPEN "v62_c5.tmp" FOR RANDOM AS #1 LEN = 8
FIELD #1, 4 AS e()
PRINT "3. after FIELD #1, 4 AS e()"
CLOSE #1
KILL "v62_c5.tmp"
PRINT "4. end"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
