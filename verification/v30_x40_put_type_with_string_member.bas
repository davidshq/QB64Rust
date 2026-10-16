$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (40_put_type_with_string_member)
ON ERROR GOTO h
OPEN "v30_x40_put_type_with_string_member.tmp" FOR BINARY AS #1
TYPE t
    n AS LONG
    s AS STRING
END TYPE
DIM r AS t
r.s = "abc"
PUT #1, 1, r
PRINT LOF(1)
CLOSE
KILL "v30_x40_put_type_with_string_member.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
