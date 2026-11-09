$CONSOLE:ONLY
' Verification (study 44, section 2 item 7): FOR ... NEXT in both branches of a single-line IF ... ELSE, as in
' the Museum's RETROQB45\QDigger\QDIGGER.BAS line 2447.
FOR dx = -1 TO 1 STEP 2
    IF dx > 0 THEN FOR j = 1 TO 3: PRINT j;: NEXT ELSE FOR j = 1 TO 2: PRINT -j;: NEXT
    PRINT
NEXT
SYSTEM
