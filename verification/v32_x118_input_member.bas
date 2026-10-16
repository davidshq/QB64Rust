$CONSOLE:ONLY
' Verification (m3-mem, by the review of the change): does the old compiler accept this? (118_input_member)
DIM m AS _MEM
OPEN "v32_x118.txt" FOR OUTPUT AS #1: PRINT #1, 4: CLOSE #1
OPEN "v32_x118.txt" FOR INPUT AS #1
INPUT #1, m.SIZE
CLOSE #1
KILL "v32_x118.txt"
PRINT "ok"
SYSTEM
