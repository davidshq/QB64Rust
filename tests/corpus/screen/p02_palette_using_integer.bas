' PALETTE USING from an INTEGER array in SCREEN 7: the 16 attributes get the EGA colours the whole array holds, an
' element of -1 leaves its attribute as it was, and boxes are drawn in each attribute.
SCREEN 7
DIM a(15) AS INTEGER
FOR i = 0 TO 15
    a(i) = (i * 5) MOD 64
NEXT
a(4) = -1
PALETTE USING a
FOR c = 0 TO 15
    LINE (c * 20, 0)-(c * 20 + 19, 90), c, BF
NEXT
SYSTEM
