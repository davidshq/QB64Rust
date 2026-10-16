$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, task 5.3): GET of a whole array of a type with a _DYNAMIC member into a dynamic array
' that ERASE released (found while s118_dyn_member_files was recorded), under a handler.
TYPE t
    a AS INTEGER
    v(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
REDIM p(1) AS t
OPEN "v35_x67.bin" FOR BINARY AS #1
PUT #1, 1, p()
PRINT "PUT: LOF ="; LOF(1)
ERASE p
GET #1, 1, p()
PRINT "GET into the erased array: done"
CLOSE #1
KILL "v35_x67.bin"
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
