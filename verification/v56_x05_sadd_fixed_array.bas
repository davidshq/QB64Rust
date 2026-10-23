$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): SADD of an element of an array of fixed-length strings.
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM i AS INTEGER, t AS STRING, fx AS STRING * 4, r AS rec, rs AS ts, bt AS _BIT, bu AS _UNSIGNED _BIT * 4
DIM fa(1 TO 3) AS STRING * 4, ra(1 TO 3) AS rec, ba(1 TO 3) AS _BIT, sa(1 TO 3) AS STRING
i = 7: t = "abc": fx = "wxyz": r.y = 9: rs.n = "member": bt = -1: bu = 5: fa(2) = "elem": sa(2) = "str"
PRINT SADD(fa(2))
SYSTEM

SUB show (v AS STRING)
    FOR k = 1 TO LEN(v): PRINT ASC(v, k);: NEXT
    PRINT
END SUB
