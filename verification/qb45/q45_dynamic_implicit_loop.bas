' QB 4.5 (beside SOMEDAY's row on an array used without DIM below $DYNAMIC, v24_x88): is the array made once, or
' by the statement that names it first, so that the statement run again raises error 10? Also an array whose
' first statement a GOTO skips, and one first named in a LOOP UNTIL line.
' $DYNAMIC
ON ERROR GOTO h
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "1. a loop storing x(i):";
FOR i = 1 TO 3
    x(i) = i * 10
NEXT
PRINT #1, x(1); x(2); x(3)
PRINT #1, "2. GOTO past the first use of y():";
GOTO skip
y(1) = 5
skip:
y(2) = 7
PRINT #1, y(2)
PRINT #1, "3. LOOP UNTIL z(n) > 2:";
n = 0
DO
    n = n + 1
LOOP UNTIL z(n) > 2 OR n = 3
PRINT #1, n
PRINT #1, "4. UBOUND(x):"; UBOUND(x)
CLOSE #1
SYSTEM
h:
PRINT #1, " error"; ERR;
RESUME NEXT
