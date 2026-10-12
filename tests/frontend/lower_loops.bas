' TEST: ir
$CONSOLE:ONLY
' WHILE and DO lowered (m2-control-flow-slice task 6.2, design D8): a test at the top is a Skip branch out of the
' loop (WHILE and DO WHILE leave on zero, DO UNTIL on non-zero), a test at the bottom a Skip branch back to the top
' (LOOP WHILE on non-zero, LOOP UNTIL on zero), otherwise a jump back that carries the WEND or LOOP line; EXIT DO
' and EXIT WHILE jump to the innermost loop's exit of their kind, also from inside an IF and past an inner loop
WHILE a < 2
    a = a + 1
    IF a = 1 THEN EXIT WHILE
WEND
DO WHILE a < 3: a = a + 1: LOOP
DO UNTIL a = 0: a = a - 1: LOOP
DO: a = a + 1: LOOP WHILE a < 3
DO: a = a - 1: LOOP UNTIL a = 0
DO
    WHILE a < 5
        a = a + 1
        EXIT DO
    WEND
LOOP
PRINT a
