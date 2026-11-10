' TEST: check-fail
$CONSOLE:ONLY
' _MEMGET, _MEMPUT and _MEMFILL of a value of a TYPE that holds a STRING: "UDT must have fixed size", as in the
' old compiler (verification v66_b01 to b03 for a variable). The element, the member and the TYPE that holds such a
' TYPE are read in qb64pe.bas 22226, not measured. The whole array is not measured and stays "not supported yet".
TYPE ts
    n AS LONG
    s AS STRING
END TYPE
TYPE outer
    k AS INTEGER
    t AS ts
END TYPE
DIM w AS ts, wa(3) AS ts, o AS outer, m AS _MEM
_MEMGET m, m.OFFSET, w
_MEMPUT m, m.OFFSET, w
_MEMFILL m, m.OFFSET, 8, w
_MEMGET m, m.OFFSET, wa(1)
_MEMGET m, m.OFFSET, o.t
_MEMPUT m, m.OFFSET, o
_MEMGET m, m.OFFSET, wa()
_MEMGET m, m.OFFSET, o.k
SYSTEM
