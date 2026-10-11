' TEST: cpp
$CONSOLE:ONLY
' `_BIT` arrays (m2-defxxx-common-arrays task 4.2, design D7; measured, verification\v24_b_bit_arrays): packed
' storage of count * bits / 8 + 1 bytes, static and dynamic; an element is read with getbits (getubits when
' unsigned) and stored with setbits at the checked flat position; REDIM _PRESERVE copies the kept elements one by
' one; an element is passed to a procedure as a copy.
n = 9
DIM b(7) AS _BIT
DIM u3(9) AS _UNSIGNED _BIT * 3
DIM dy(n) AS _BIT * 5
b(1) = 1
u3(n) = u3(2) + b(1)
REDIM _PRESERVE dy(n + 1) AS _BIT * 5
dy(2) = 2.5
show dy(2)
ERASE b, dy
SYSTEM
SUB show (v AS LONG)
    PRINT v
END SUB
