$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 5, BLOAD of a file whose header gives size 0 and whose data is
' 200000 bytes long (libqb then reads the whole file), to &HF000:0, 131059 bytes before the end of the block.
ON ERROR GOTO h
DIM hdr AS STRING, body AS STRING
hdr = CHR$(253) + MKI$(0) + MKI$(0) + MKI$(0)
body = STRING$(200000, 65)
OPEN "v63_b2.tmp" FOR BINARY AS #1
PUT #1, , hdr
PUT #1, , body
CLOSE #1
DEF SEG = &HF000
BLOAD "v63_b2.tmp", 0
PRINT "after BLOAD"
DEF SEG
KILL "v63_b2.tmp"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
