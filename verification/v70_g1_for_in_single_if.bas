$CONSOLE:ONLY
' Verification (study 44, section 2 item 7): a whole FOR ... NEXT inside a single-line IF.
IF 1 THEN FOR i = 1 TO 3: PRINT i;: NEXT: PRINT
IF 1 THEN FOR i = 1 TO 2: PRINT i;: NEXT i: PRINT
IF 0 THEN FOR i = 1 TO 2: PRINT i;: NEXT
PRINT "end"
SYSTEM
