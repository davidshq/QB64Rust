' TEST: ir
$CONSOLE:ONLY
' _CONTINUE in the IR (m4-devices-continue-paths task 2.2, design D1; verification\v47_a_continue): a jump to a
' label placed just before the loop's bottom statement: before the step of NEXT, before the test of LOOP UNTIL,
' before the jump back of WEND; from a single-line IF and a SELECT, to the innermost loop. Two _CONTINUEs of one
' loop share its label; a loop without one gets none.
DIM i AS LONG, n AS LONG
FOR i = 1 TO 4
    IF i = 2 THEN _CONTINUE
    PRINT i
NEXT
DO
    n = n + 1
    SELECT CASE n
        CASE 1: _CONTINUE
        CASE 2: _CONTINUE
    END SELECT
    PRINT n
LOOP UNTIL n >= 5
WHILE n > 0
    n = n - 1
    FOR i = 1 TO 2
        PRINT i
    NEXT
    IF n = 2 THEN _CONTINUE
    PRINT n
WEND
