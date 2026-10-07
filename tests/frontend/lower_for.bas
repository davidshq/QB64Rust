' TEST: ir
$CONSOLE:ONLY
' FOR lowered (m2-control-flow-slice task 6.2, design D8, study\02 6.5): one header statement stores the count,
' limit, step and the step's sign (AssignAll) and jumps to the entry; the body follows, then the step added to the
' variable's current value, then the entry stores the count into the variable, leaves when past the limit and
' jumps back to the body. Temporaries are wider than the variable (INTEGER counts in LONG, SINGLE in DOUBLE), the
' default step is 1. NEXT j, i closes two loops on one line; EXIT FOR inside an IF leaves the inner loop only;
' GOTO into a loop body from outside jumps to a user label inside it; a raising limit is stored like the others
FOR i% = 1 TO 10 STEP 2
    PRINT i%
NEXT
FOR s = 0 TO 1 STEP 0.5: NEXT s
FOR j = 1 TO 3
    FOR k = 1 TO 3
        IF k = 2 THEN EXIT FOR
        IF j = 3 THEN GOTO inside
    NEXT k, j
GOTO inside
FOR m = 1 TO INSTR(CHR$(-1), "a") + 2
    inside:
    PRINT m
NEXT
