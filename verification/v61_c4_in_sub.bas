$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): CALL ABSOLUTE in a SUB as tictac of qbasic_testcases calls it: the
' code in a STATIC string, its offset SADD of the string with no DEF SEG, a literal, a DOUBLE value and two DIM
' SHARED variables as arguments; the code copies the literal's low word and the DOUBLE's high word into them.
DIM SHARED click AS INTEGER, cv AS INTEGER
mouse
PRINT click; cv
mouse
SYSTEM

SUB mouse
    STATIC mx AS STRING
    IF mx = "" THEN
        FOR i% = 1 TO 28
            READ b%
            mx = mx + CHR$(b%)
        NEXT
    END IF
    CALL ABSOLUTE(2, 2.5#, click, cv, SADD(mx))
    PRINT click; cv
END SUB
' push bp; mov bp,sp
DATA &H55,&H8B,&HEC
' mov bx,[bp+12]; mov ax,[bx]; mov bx,[bp+8]; mov [bx],ax
DATA &H8B,&H5E,&H0C,&H8B,&H07,&H8B,&H5E,&H08,&H89,&H07
' mov bx,[bp+10]; mov ax,[bx+6]; mov bx,[bp+6]; mov [bx],ax
DATA &H8B,&H5E,&H0A,&H8B,&H47,&H06,&H8B,&H5E,&H06,&H89,&H07
' pop bp; retf 8
DATA &H5D,&HCA,&H08,&H00
