$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 8, the block of a dynamic array after CLEAR releases it.
' CLEAR clears every variable, so the block is kept in a file and read back after it; then _MEMEXISTS and a
' _MEMPUT through it (errors 300 to 315 are not trapped).
ON ERROR GOTO h
DIM m AS _MEM
REDIM d(5) AS LONG
m = _MEM(d())
PRINT "1."; _MEMEXISTS(m)
OPEN "v62_d3.tmp" FOR BINARY AS #1
PUT #1, 1, m
CLOSE #1
CLEAR
DIM m2 AS _MEM
OPEN "v62_d3.tmp" FOR BINARY AS #1
GET #1, 1, m2
CLOSE #1
KILL "v62_d3.tmp"
PRINT "2."; _MEMEXISTS(m2)
_MEMPUT m2, m2.OFFSET, 1 AS LONG
PRINT "3. after _MEMPUT"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
