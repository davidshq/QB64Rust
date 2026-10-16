' TEST: ir
$CONSOLE:ONLY
' GET and PUT on a file, LOCK and UNLOCK in the IR (m3-console-files tasks 5.1, 5.2, design D2, D4;
' verification\v30_d_binary, v30_e_random, v30_i_lock, v30_x20-x56, x67): the file form of GET and PUT is the
' statement's second table entry, with the file number (a LONG), the position (an _INTEGER64, a float rounded) or
' nothing, and the bytes of the variable or nothing. The graphics form keeps its own entry (the last two lines)
TYPE t
    n AS LONG
    s AS STRING * 6
    v AS STRING
END TYPE
TYPE fixed
    n AS LONG
    s AS STRING * 6
END TYPE
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, d AS DOUBLE, f AS _FLOAT, v AS STRING, x AS STRING * 6
DIM r AS t, w AS fixed, a(1 TO 4) AS LONG, e(3) AS STRING, fx(2) AS STRING * 4, wa(2) AS fixed, k AS LONG
' A numeric variable, with and without a position, with a position of each kind
GET #1, , i
GET #1, 3, l
PUT #1, , q
PUT #k + 1, d, f
GET 1, q, i
' Strings: variable-length (the string itself), fixed-length (its characters)
GET #1, , v
PUT #1, 2, v
GET #1, , x
PUT #1, 2, x
' A TYPE variable and members of each kind
PUT #1, , w
GET #1, 1, w
GET #1, , r.n
PUT #1, , r.s
GET #1, , r.v
' Elements: numbers, strings of both kinds, a TYPE, a member of an element
GET #1, , a(2)
PUT #1, k, a(k)
GET #1, , e(1)
PUT #1, , e(k)
GET #1, , fx(1)
PUT #1, , wa(1)
GET #1, , wa(k).s
' A whole array: numbers, fixed-length strings, a TYPE
GET #1, , a()
PUT #1, 1, fx()
GET #1, 1, wa()
' No variable: the file's record buffer; a variable in parentheses; a name alone is a scalar
GET #1
GET #1, 2
PUT #1,
PUT #1, k
PUT #1, , (i)
GET #1, , fresh
' After CALL, with and without the number sign
CALL GET(1, 1, i)
CALL PUT(#1, , v)
CALL GET(#1)
' LOCK and UNLOCK: the file alone, one record, a range, a comma and nothing more; after CALL
LOCK #1
LOCK #1, 2
LOCK #1, 2 TO 4
LOCK #1,
UNLOCK k, d TO q
UNLOCK #1
CALL LOCK(1, 2 TO 3)
CALL UNLOCK(#1, 2 TO 3)
' The graphics form
GET (0, 0)-(3, 3), a
PUT (0, 0), a, PSET
SYSTEM
