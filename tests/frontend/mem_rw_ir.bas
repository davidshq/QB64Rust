' TEST: ir
$CONSOLE:ONLY
' _MEMGET, _MEMPUT and _MEMFILL as statements (m3-mem task 4.1, design D4; verification\v32_x01_accepted): the
' block, the offset (and count) as _OFFSET values, and what is read into or written from: a variable, an element,
' a member, a whole array, a whole member array, a string that is no variable, a value AS a numeric type.
TYPE t
    a AS INTEGER
    v(0 TO 3) AS LONG
END TYPE
DIM m AS _MEM, z(2) AS _MEM, u AS t, k AS LONG, d AS DOUBLE, s AS STRING, b(5) AS INTEGER
_MEMGET m, m.OFFSET, k
_MEMGET z(1), 4, b(2)
_MEMGET m, k, s
_MEMPUT m, m.OFFSET, u.v()
_MEMPUT m, m.OFFSET, s + "x"
_MEMPUT m, m.OFFSET, d AS _INTEGER64
_MEMPUT m, m.OFFSET, k + 1 AS INTEGER
_MEMFILL m, m.OFFSET, 8, 3 AS _FLOAT
_MEMFILL m, m.OFFSET, k, b()
_MEMFILL m, m.OFFSET, d, "ab"
SYSTEM
