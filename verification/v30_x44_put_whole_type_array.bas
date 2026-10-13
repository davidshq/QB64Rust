$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (44_put_whole_type_array)
ON ERROR GOTO h
OPEN "v30_x44_put_whole_type_array.tmp" FOR BINARY AS #1
TYPE t
    n AS LONG
    m AS INTEGER
END TYPE
DIM ra(1 TO 3) AS t
PUT #1, 1, ra()
PRINT LOF(1)
PUT #1, 1, ra(2)
PRINT SEEK(1)
CLOSE
KILL "v30_x44_put_whole_type_array.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
