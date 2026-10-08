$CONSOLE:ONLY
' ERASE of a member array written with blanks around the dot (the upstream tests' form, `ERASE a . S`), and of a
' plain array, then REDIM of the member array again.
TYPE t
    s(1 TO 3) AS LONG
END TYPE
DIM a AS t
DIM b(2) AS LONG
a.s(2) = 7
b(1) = 4
PRINT a.s(2); b(1)
ERASE a . S, b
PRINT a.s(2); b(1)
SYSTEM
