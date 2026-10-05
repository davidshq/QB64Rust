' TEST: parse-ok
$CONSOLE:ONLY
' FOR, DO and WHILE loops and the EXIT forms the old compiler accepts (m2-parser-breadth task 6.2; measured in
' verification\v16_m4_next_two, v16_m4_loop_forms, v16_m4_exit_forms)
FOR i = 1 TO 2
    FOR j = 1 TO 2
        PRINT i; j
NEXT j, i
FOR k = 1 TO 2
    PRINT "k"; k
NEXT
FOR k% = 10 TO 1 STEP -3: PRINT k%;: NEXT k%
i = 0
DO WHILE i < 2: i = i + 1: LOOP
DO UNTIL i = 4: i = i + 1: LOOP
DO: i = i + 1: LOOP WHILE i < 6
DO: i = i + 1: LOOP UNTIL i = 8
WHILE i > 0
    i = i - 1
    IF i = 5 THEN EXIT WHILE
WEND
FOR i = 1 TO 3
    DO
        IF i = 2 THEN EXIT FOR
        PRINT "i"; i
        EXIT DO
    LOOP
NEXT
DO
    FOR j = 1 TO 3
        IF j = 2 THEN EXIT DO
        PRINT "j"; j
    NEXT
LOOP
SUB s
    FOR n = 1 TO 2
        IF n = 2 THEN EXIT SUB
    NEXT
END SUB
