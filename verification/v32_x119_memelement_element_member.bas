$CONSOLE:ONLY
' Verification (m3-mem, by the review of the change): the block of _MEMELEMENT of a member of an element of a
' user-type array, of an element of a member array of such an element, and of a member array of a member of an
' element, printed as v32_a prints a block: OFFSET less the address of the array's first element, SIZE, TYPE,
' ELEMENTSIZE, IMAGE and SOUND.
TYPE t
    a AS INTEGER
    s AS STRING * 5
    v(0 TO 3) AS LONG
    d AS DOUBLE
END TYPE
DIM ut(3) AS t, m AS _MEM
m = _MEMELEMENT(ut(1).d): show "ut(1).d", m, _OFFSET(ut())
m = _MEMELEMENT(ut(1).a): show "ut(1).a", m, _OFFSET(ut())
m = _MEMELEMENT(ut(1).s): show "ut(1).s", m, _OFFSET(ut())
m = _MEMELEMENT(ut(1).v(2)): show "ut(1).v(2)", m, _OFFSET(ut())
SYSTEM

SUB show (l$, m AS _MEM, bas AS _OFFSET)
    PRINT l$; ":"; m.OFFSET - bas; m.SIZE; m.TYPE; m.ELEMENTSIZE; m.IMAGE; m.SOUND
END SUB
