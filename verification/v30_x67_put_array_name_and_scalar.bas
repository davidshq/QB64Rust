$CONSOLE:ONLY
' Verification (step 4 of M3, task 5.1 of m3-console-files): does the old compiler accept this, and what does it
' do? (67_put_array_name_and_scalar) An array's name with no parentheses as the variable of PUT: the whole array
' (6 bytes), also when a scalar of the name exists (4 bytes)? And a name that is neither yet.
ON ERROR GOTO h
DIM a(1 TO 3) AS INTEGER, b(1 TO 3) AS INTEGER, b AS LONG
OPEN "v30_x67.tmp" FOR BINARY AS #1
PUT #1, 1, a
PRINT LOF(1)
CLOSE #1: KILL "v30_x67.tmp"
OPEN "v30_x67.tmp" FOR BINARY AS #1
PUT #1, 1, b
PRINT LOF(1)
CLOSE #1: KILL "v30_x67.tmp"
OPEN "v30_x67.tmp" FOR BINARY AS #1
PUT #1, 1, fresh
PRINT LOF(1)
PUT #1, 1, fresh$
PRINT LOF(1)
CLOSE #1: KILL "v30_x67.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
