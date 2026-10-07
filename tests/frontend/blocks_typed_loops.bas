' TEST: typed
$CONSOLE:ONLY
' FOR, DO and WHILE as typed blocks (m2-control-flow-slice task 5.2, design D3): the type a FOR loop counts in for
' each variable type (study\02 6.5, measured in task 1.1), start/limit/step converted to it; NEXT j, i closing
' two loops; NEXT with the variable's suffix; DO in its five forms; WHILE; EXIT FOR/DO/WHILE
DIM l AS LONG
FOR i% = 1 TO 2.6: NEXT
FOR l = 1 TO 2: NEXT l&
FOR q&& = 1 TO 2: NEXT
FOR s = 0 TO 1 STEP 0.1: NEXT s!
FOR d# = 1 TO 2: NEXT d#
FOR f## = 1 TO 2: NEXT
FOR j = 1 TO 3
    FOR k = 1 TO 3
        IF k = 2 THEN EXIT FOR
    NEXT k, j
DO
    EXIT DO
LOOP
DO WHILE a < 3: a = a + 1: LOOP
DO UNTIL a = 0: a = a - 1: LOOP
DO: a = a + 1: LOOP WHILE a < 3
DO: a = a - 1: LOOP UNTIL a = 0
WHILE a < 2
    a = a + 1
    IF a = 1 THEN EXIT WHILE
WEND
