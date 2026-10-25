$CONSOLE:ONLY
' Verification (m4-left-marked, task 1.1): FOR over r.m(i) whose index leaves the member array's bounds in the
' body, under a handler that resumes next: the error at NEXT, and where the loop goes on.
TYPE t
    m(3) AS INTEGER
END TYPE
ON ERROR GOTO h
DIM r AS t, i AS INTEGER, k AS INTEGER
i = 1
FOR r.m(i) = 1 TO 3
    k = k + 1
    PRINT " pass"; k;
    IF k > 4 THEN EXIT FOR
    i = 7
NEXT
PRINT
PRINT "after:"; r.m(1); i; k
i = 0: k = 0
PRINT "start out of range:";
FOR r.m(i - 1) = 1 TO 2
    k = k + 1
    PRINT " body";
    IF k > 4 THEN EXIT FOR
NEXT
PRINT " after"; k
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
