' QB 4.5 (DIVERGENCES-QB45.md Q-004): INTEGER arithmetic beyond the INTEGER range inside an
' expression. QB64pe computes it in 32 bits (line 1 prints 32768) and wraps only the store
' into an INTEGER. Lines 5 and 6 are the same for LONG (QB64PE-DIVERGENCES.md D-001).
ON ERROR GOTO h
DIM i AS INTEGER, j AS INTEGER, l AS LONG
OPEN "OUT.TXT" FOR OUTPUT AS #1
i = 32767
PRINT #1, "1."; i + 1
PRINT #1, "2.";
j = i + 1
PRINT #1, j
PRINT #1, "3."; i * 2
PRINT #1, "4.";
l = i + 1
PRINT #1, l
l = 2147483647
PRINT #1, "5."; l + 1
PRINT #1, "6.";
l = l + 1
PRINT #1, l
PRINT #1, "end"
CLOSE #1
SYSTEM
h:
PRINT #1, "[error"; ERR; "]";
RESUME NEXT
