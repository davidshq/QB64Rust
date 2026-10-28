$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 5, BLOAD with no offset of a file whose header names &HFFFF:&HFFF0
' and a size of 1000, so it is read to the place the header names, past the end of the block.
ON ERROR GOTO h
DIM hdr AS STRING, body AS STRING
hdr = CHR$(253) + MKI$(&HFFFF) + MKI$(&HFFF0) + MKI$(1000)
body = STRING$(1000, 66)
OPEN "v63_b4.tmp" FOR BINARY AS #1
PUT #1, , hdr
PUT #1, , body
CLOSE #1
BLOAD "v63_b4.tmp"
PRINT "after BLOAD"
KILL "v63_b4.tmp"
PRINT "done"
SYSTEM
h:
PRINT " [error"; ERR; "]"
RESUME NEXT
