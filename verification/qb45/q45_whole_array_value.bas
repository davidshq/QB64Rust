' QB 4.5 (row 6 of the crash-class rulings, study\39): a whole array a() used as a value in
' an expression, which QB64pe takes as the array's first element (verification\v54_c_whole_values).
DIM a(5 TO 7) AS LONG
a(5) = 55: a(6) = 66: a(7) = 77
OPEN "OUT.TXT" FOR OUTPUT AS #1
PRINT #1, "1."; a()
PRINT #1, "2."; a() + 1
CLOSE #1
SYSTEM
