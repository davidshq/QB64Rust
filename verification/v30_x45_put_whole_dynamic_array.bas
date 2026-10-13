$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (45_put_whole_dynamic_array)
ON ERROR GOTO h
OPEN "v30_x45_put_whole_dynamic_array.tmp" FOR BINARY AS #1
DIM n AS LONG
n = 5
DIM da(1 TO n) AS LONG
PUT #1, 1, da()
PRINT LOF(1)
REDIM da(1 TO 2) AS LONG
GET #1, 1, da()
PRINT SEEK(1)
CLOSE
KILL "v30_x45_put_whole_dynamic_array.tmp"
PRINT "ok"
SYSTEM
h:
PRINT "<error"; ERR; ">"
RESUME NEXT
