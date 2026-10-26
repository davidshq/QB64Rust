$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): CALL ABSOLUTE of code that resets the mouse through INT 33h (AX = 0),
' which libqb's emulator answers (mouse installed, three buttons), and stores AX and BX through its arguments.
DIM code(15) AS INTEGER
DEF SEG = VARSEG(code(0))
FOR i = 0 TO 21
    READ b
    POKE VARPTR(code(0)) + i, b
NEXT
CALL ABSOLUTE(a%, b%, VARPTR(code(0)))
DEF SEG
PRINT a%; b%
SYSTEM
' push bp; mov bp,sp; mov ax,0; int 33h
DATA &H55,&H8B,&HEC,&HB8,&H00,&H00,&HCD,&H33
' mov si,[bp+8]; mov [si],ax; mov si,[bp+6]; mov [si],bx
DATA &H8B,&H76,&H08,&H89,&H04,&H8B,&H76,&H06,&H89,&H1C
' pop bp; retf 4
DATA &H5D,&HCA,&H04,&H00
