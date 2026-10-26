$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): CALL ABSOLUTE of machine code that stores 1234 through its argument's
' offset: push bp; mov bp,sp; mov bx,[bp+6]; mov word [bx],1234; pop bp; retf 2.
DIM code(15) AS INTEGER
DEF SEG = VARSEG(code(0))
FOR i = 0 TO 13
    READ b
    POKE VARPTR(code(0)) + i, b
NEXT
v% = 7
CALL ABSOLUTE(v%, VARPTR(code(0)))
DEF SEG
PRINT v%
SYSTEM
DATA &H55,&H8B,&HEC,&H8B,&H5E,&H06,&HC7,&H07,&HD2,&H04,&H5D,&HCA,&H02,&H00
