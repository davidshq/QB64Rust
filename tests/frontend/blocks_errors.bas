' TEST: check-fail
$CONSOLE:ONLY
' Errors of the typed blocks (m2-control-flow-slice task 5.2, design D3, D10), each rejected by the old compiler
' (verification\v17_c_err_*, v17_c_next_suffix, v17_c_for_array); the statements inside a block with a bad header
' are still checked (line 10). A TYPE member as the FOR variable is accepted by the old compiler
' (v17_c_for_type_member): not supported yet here.
CONST c = 1
IF "a" THEN PRINT 1
IF 1 THEN
    x = "string in a number"
ELSEIF "b" THEN
END IF
WHILE "c": WEND
DO WHILE "d": LOOP
DO: LOOP UNTIL "e"
FOR s$ = 1 TO 2: NEXT
FOR c = 1 TO 2: NEXT
FOR n = 1 TO "x": NEXT
FOR n = 1 TO 2 STEP "y": NEXT
FOR i = 1 TO 2: NEXT j
FOR i% = 1 TO 2: NEXT i
FOR i = 1 TO 2
    FOR j = 1 TO 2
    NEXT i, j
FOR arr(1) = 1 TO 3: NEXT
FOR t.x = 1 TO 2: NEXT
