$CONSOLE:ONLY
' Verification (m4-names-arrays, task 1.1): TYPE blocks in a SUB, a FUNCTION, an IF block and a FOR loop,
' each a type of the whole program.
PRINT "1. a type of a SUB used above it in the main module:";
DIM q AS pt
q.x = 3: q.y = 4
PRINT q.x; q.y; LEN(q)
PRINT "2. as another SUB's parameter type and a FUNCTION's:";
useit q
PRINT getx(q)
PRINT "3. the SUB itself:";
s1
PRINT "4. a type of an IF block, used above it:";
DIM r AS inif
r.v = 4: PRINT r.v; LEN(r)
IF 0 THEN
    TYPE inif
        v AS _INTEGER64
    END TYPE
END IF
PRINT "5. a type of a FOR loop, used below it:";
FOR k = 1 TO 1
    TYPE infor
        w AS INTEGER
        n AS STRING * 3
    END TYPE
NEXT
DIM w AS infor: w.w = 5: w.n = "abc"
PRINT w.w; w.n; LEN(w)
PRINT "6. a type of a FUNCTION, used in the main module:";
DIM fr AS infn: fr.z = 2.5
PRINT fr.z; LEN(fr)
PRINT "7. a type of a SUB never called, in a SELECT CASE:";
DIM sc AS insel: sc.b = -1
PRINT sc.b; LEN(sc)
PRINT "8. an array of a SUB's type:";
DIM qa(2) AS pt
qa(2).y = 9
PRINT qa(2).y; LEN(qa(1))
SYSTEM

SUB s1
    TYPE pt
        x AS LONG
        y AS LONG
    END TYPE
    DIM p AS pt
    p.x = 5: PRINT p.x; LEN(p)
END SUB

SUB useit (p AS pt)
    PRINT "useit"; p.x;
END SUB

FUNCTION getx& (p AS pt)
    TYPE infn
        z AS DOUBLE
    END TYPE
    getx& = p.x * 2
END FUNCTION

SUB never (n)
    SELECT CASE n
        CASE 1
            TYPE insel
                b AS _BYTE
            END TYPE
    END SELECT
END SUB
