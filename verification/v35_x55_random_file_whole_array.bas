$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): does the old compiler accept this? (55_random_file_whole_array)
TYPE t
    n AS INTEGER
    v(1) _DYNAMIC AS LONG
END TYPE
REDIM p(1) AS t
p(1).n = 5: p(1).v(1) = 7
ON ERROR GOTO h
OPEN "v35_x55.tmp" FOR RANDOM AS #1 LEN = 10
PUT #1, 1, p()
PRINT LOF(1)
CLOSE #1
OPEN "v35_x55.tmp" FOR RANDOM AS #1 LEN = 20
PUT #1, 1, p()
PRINT LOF(1)
CLOSE #1
KILL "v35_x55.tmp"
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
SYSTEM
