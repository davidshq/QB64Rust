$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): _CONTINUE as a single-line IF's ELSE.
FOR i = 1 TO 4
    IF i <> 3 THEN PRINT i; ELSE _CONTINUE
    PRINT "+";
NEXT
PRINT
SYSTEM
