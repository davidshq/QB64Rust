$CONSOLE:ONLY
' v68 "Unused variable": the name and type qb64pe -w prints for each type, one unused variable per line.
TYPE pt
    x AS INTEGER
END TYPE
DIM MixedCase AS INTEGER
DIM ul AS _UNSIGNED LONG
DIM fs AS STRING * 5
DIM vs AS STRING
DIM p AS pt
DIM bt AS _BIT
DIM b3 AS _BIT * 3
DIM ub AS _UNSIGNED _BYTE
DIM by AS _BYTE
DIM i6 AS _INTEGER64
DIM fl AS _FLOAT
DIM sg AS SINGLE
DIM o AS _OFFSET
DIM uo AS _UNSIGNED _OFFSET
DIM mm AS _MEM
DIM sfx&
DIM ar(2) AS pt
PRINT "types"
SYSTEM
