$CONSOLE:ONLY
' _CONTINUE (m4-devices-continue-paths, design D1; verification\v47_a_continue): the next pass of the innermost
' FOR, DO or WHILE loop, from a single-line IF and from a SELECT CASE (as upstream colors/color_with_dest), with the
' step of NEXT and the test of LOOP UNTIL still run; in a SUB.
FOR i = 1 TO 7
    SELECT CASE i
        CASE 3, 6: _CONTINUE
    END SELECT
    PRINT i;
NEXT
PRINT
n = 0
DO
    n = n + 1
    IF n < 3 THEN _CONTINUE
    PRINT n;
LOOP UNTIL n >= 5
PRINT
n = 0
WHILE n < 4
    n = n + 1
    FOR j = 1 TO 3
        IF j = 2 THEN _CONTINUE
        PRINT n * 10 + j;
    NEXT
    IF n = 2 THEN _CONTINUE
    PRINT "/";
WEND
PRINT
odd 6
SYSTEM

SUB odd (m)
    FOR k = 1 TO m
        IF k MOD 2 = 0 THEN _CONTINUE
        PRINT k;
    NEXT
    PRINT
END SUB
