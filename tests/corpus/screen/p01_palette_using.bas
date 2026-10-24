' PALETTE USING from a LONG array in SCREEN 13 (libqb takes only LONG there): every attribute gets the levels the
' array holds, from its element 2, and lines are drawn in colours it changed.
SCREEN 13
DIM b(257) AS LONG
FOR i = 0 TO 255
    b(i + 2) = 65536 * (i MOD 64) + 256 * ((i \ 4) MOD 64) + (63 - i MOD 64)
NEXT
PALETTE USING b(2)
FOR c = 0 TO 15
    LINE (c * 20, 0)-(c * 20 + 19, 90), c * 16 + 3, BF
NEXT
SYSTEM
