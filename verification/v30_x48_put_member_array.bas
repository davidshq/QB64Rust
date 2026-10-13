$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (48_put_member_array)
ON ERROR GOTO h
OPEN "v30_x48_put_member_array.tmp" FOR BINARY AS #1
TYPE t
    v(1 TO 3) AS INTEGER
END TYPE
DIM r AS t
PUT #1, 1, r.v()
PRINT LOF(1)
PUT #1, 1, r.v(2)
PRINT SEEK(1)
CLOSE
KILL "v30_x48_put_member_array.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
