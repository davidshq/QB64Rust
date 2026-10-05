$CONSOLE:ONLY
' Verification (m2-parser-breadth, M2): READ into numbers from empty items, blanks around numbers, and &H.
DATA , 5 ,  -2.5E1 , &H10,
READ a, b, c, d, e
PRINT a; b; c; d; e
SYSTEM
