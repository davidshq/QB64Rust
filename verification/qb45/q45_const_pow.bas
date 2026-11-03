' QB 4.5 (DIVERGENCES-QB45.md Q-008): a chain of ^ in a CONST, a single ^ in a CONST, and the
' chain at run time and in a PRINT of literals. QB64pe gives 512 for the chain in a CONST and
' 64 at run time.
CONST c = 2 ^ 3 ^ 2
CONST b = 2 ^ 3
OPEN "OUT.TXT" FOR OUTPUT AS #1
x = 2: y = 3: z = 2
PRINT #1, "const chain:"; c
PRINT #1, "const single:"; b
PRINT #1, "run:"; x ^ y ^ z
PRINT #1, "literals:"; 2 ^ 3 ^ 2
CLOSE #1
SYSTEM
