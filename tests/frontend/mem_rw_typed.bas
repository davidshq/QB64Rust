' TEST: typed
$CONSOLE:ONLY
' The _MEMGET function (m3-mem task 4.1, design D4; verification\v32_x01_accepted): a value of each numeric type,
' of STRING * n (a STRING value) and of a user type (stored into a place of it) at an offset stored as an _OFFSET;
' the block a variable, an element or a member of type _MEM.
TYPE t
    a AS INTEGER
    d AS DOUBLE
    mm AS _MEM
END TYPE
DIM m AS _MEM, z(2) AS _MEM, u AS t, k AS LONG, d AS DOUBLE, f AS STRING * 4, o AS _OFFSET, fl AS _FLOAT
k = _MEMGET(m, o, LONG)
d = _MEMGET(m, m.OFFSET + 8, DOUBLE)
PRINT _MEMGET(m, o, _UNSIGNED _BYTE); _MEMGET(z(1), 5, INTEGER); _MEMGET(u.mm, k, _UNSIGNED _OFFSET)
f = _MEMGET(m, o, STRING * 4)
PRINT _MEMGET(m, o, STRING * 2) + "!"
u = _MEMGET(m, o, t)
fl = _MEMGET(m, o, _FLOAT) + 1
z(2) = _MEMGET(m, o, _MEM)
SYSTEM
