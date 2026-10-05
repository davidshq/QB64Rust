$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4): single-line IF with ELSE and nested IFs; which IF does each ELSE belong to?
FOR a = 0 TO 1
    FOR b = 0 TO 1
        PRINT a; b; ":";
        IF a THEN IF b THEN PRINT "A" ELSE PRINT "B" ELSE PRINT "C"
        PRINT a; b; ":";
        IF a THEN IF b THEN PRINT "D" ELSE PRINT "E"
        PRINT "|"
    NEXT
NEXT
IF 1 THEN PRINT "x": PRINT "y" ELSE PRINT "z": PRINT "w"
IF 0 THEN PRINT "x": PRINT "y" ELSE PRINT "z": PRINT "w"
IF 1 THEN 10 ELSE 20
10 PRINT "then 10"
IF 0 GOTO 20
PRINT "if goto not taken"
20 PRINT "twenty"
SYSTEM
