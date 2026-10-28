' QB 4.5 (row 12 of the crash-class rulings, study\39; DIVERGENCES-QB45.md Q-003): division by
' zero with an ON ERROR handler: integer \ and MOD, ERROR 11, and a float / for comparison.
ON ERROR GOTO h
DIM i AS INTEGER, z AS INTEGER, x AS SINGLE
OPEN "OUT.TXT" FOR OUTPUT AS #1
i = 7: z = 0
PRINT #1, "1."; i \ z
PRINT #1, "2."; i MOD z
PRINT #1, "3."
ERROR 11
x = 1 / z
PRINT #1, "4."; x
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]";
RESUME NEXT
