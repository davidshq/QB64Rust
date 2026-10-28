$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 8, the block of a dynamic array after RUN releases it. RUN
' clears every variable, so the block is kept in a file and read back in the second run; then _MEMEXISTS and a
' _MEMPUT through it (errors 300 to 315 are not trapped).
ON ERROR GOTO h
DIM m AS _MEM
IF _FILEEXISTS("v62_d4.tmp") THEN
    OPEN "v62_d4.tmp" FOR BINARY AS #1
    GET #1, 1, m
    CLOSE #1
    KILL "v62_d4.tmp"
    PRINT "2."; _MEMEXISTS(m)
    _MEMPUT m, m.OFFSET, 1 AS LONG
    PRINT "3. after _MEMPUT"
    SYSTEM
END IF
REDIM d(5) AS LONG
m = _MEM(d())
PRINT "1."; _MEMEXISTS(m)
OPEN "v62_d4.tmp" FOR BINARY AS #1
PUT #1, 1, m
CLOSE #1
RUN
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
