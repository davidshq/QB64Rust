' QB 4.5, PDS 7.1 and QB64: the numbers RND gives after RANDOMIZE with a fixed seed.
' The Wars of Napoleon port (Hutsell-Games/wars-of-napoleon, docs\port-study.md step 0) holds
' the QB64 builds to sessions recorded from the DOS game, which was seeded with RANDOMIZE of the
' DOUBLE 1.0. Each RND is written as the 4 bytes of its SINGLE (MKS$), so equal lines are equal
' bits. Three seedings: RANDOMIZE 1#, RANDOMIZE VAL("1") as the port's hook does, and
' RANDOMIZE 2#; then 1000 numbers after RANDOMIZE 1#, 10 to a line.
DEFINT A-Z
DECLARE FUNCTION hx$ (v!)
OPEN "OUT.TXT" FOR OUTPUT AS #1
RANDOMIZE 1#
l$ = "": FOR k = 1 TO 5: l$ = l$ + hx$(RND) + " ": NEXT k
PRINT #1, "1#   "; l$
RANDOMIZE VAL("1")
l$ = "": FOR k = 1 TO 5: l$ = l$ + hx$(RND) + " ": NEXT k
PRINT #1, "VAL  "; l$
RANDOMIZE 2#
l$ = "": FOR k = 1 TO 5: l$ = l$ + hx$(RND) + " ": NEXT k
PRINT #1, "2#   "; l$
RANDOMIZE 1#
FOR r = 1 TO 100
l$ = ""
FOR k = 1 TO 10: l$ = l$ + hx$(RND) + " ": NEXT k
PRINT #1, l$
NEXT r
CLOSE #1
SYSTEM

FUNCTION hx$ (v!)
b$ = MKS$(v!): t$ = ""
FOR i = 4 TO 1 STEP -1
h$ = HEX$(ASC(MID$(b$, i, 1))): IF LEN(h$) < 2 THEN h$ = "0" + h$
t$ = t$ + h$
NEXT i
hx$ = t$
END FUNCTION
