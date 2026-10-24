' TEST: cpp
$CONSOLE:ONLY
' RUN in each form (m4-statements-metacommands, design D5; verification\v57_r01-r19): from the top, a label, a line
' number and a file, in the main module; from the top and a label in a SUB.
IF x = 1 THEN RUN
IF x = 2 THEN RUN again
IF x = 3 THEN RUN 100
IF x = 4 THEN RUN "other.exe"
s
SYSTEM
again:
PRINT "again"
100 PRINT "100"
SUB s
    IF y = 1 THEN RUN
    RUN again
END SUB
