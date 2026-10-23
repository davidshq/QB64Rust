$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): _MEM and _MEMELEMENT of a variable, an element, a member and a
' whole array of a TYPE with a STRING member, and of a TYPE that holds such a TYPE: SIZE, ELEMENTSIZE and TYPE.
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
TYPE tw
    a AS _BYTE
    r AS ts
    f AS STRING * 3
END TYPE
DIM a(3) AS ts, v AS ts, w AS tw, wa(1 TO 2) AS tw, m AS _MEM
m = _MEM(a()): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEM(a(2)): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEMELEMENT(a(1)): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEM(v): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEM(v.v): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEM(w): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEM(w.r): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEM(wa()): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
m = _MEM(wa(2)): PRINT m.SIZE; m.ELEMENTSIZE; m.TYPE
SYSTEM
