$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): CALL ABSOLUTE with two INTEGER variables, a string variable and a
' SINGLE value: the code copies the high word of the SINGLE's slot into the first and the first word at the
' string's offset into the second.
DIM code(31) AS INTEGER
DEF SEG = VARSEG(code(0))
FOR i = 0 TO 27
    READ b
    POKE VARPTR(code(0)) + i, b
NEXT
s$ = "hello"
CALL ABSOLUTE(r%, q%, s$, 1.5!, VARPTR(code(0)))
DEF SEG
PRINT r%; q%; s$
SYSTEM
' push bp; mov bp,sp
DATA &H55,&H8B,&HEC
' mov bx,[bp+6]; mov ax,[bx+2]; mov bx,[bp+12]; mov [bx],ax
DATA &H8B,&H5E,&H06,&H8B,&H47,&H02,&H8B,&H5E,&H0C,&H89,&H07
' mov bx,[bp+8]; mov ax,[bx]; mov bx,[bp+10]; mov [bx],ax
DATA &H8B,&H5E,&H08,&H8B,&H07,&H8B,&H5E,&H0A,&H89,&H07
' pop bp; retf 8
DATA &H5D,&HCA,&H08,&H00
