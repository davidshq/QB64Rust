' TEST: check-fail
$CONSOLE:ONLY
' The forms of GET and PUT on a file and of LOCK and UNLOCK the old compiler rejects (m3-console-files tasks 5.1,
' 5.2; verification\v30_x20-x22, x24-x26, x28-x36, x38, x40, x48, x50-x55), one line each: each is an error that
' is not "not supported yet"
TYPE t
    n AS LONG
    s AS STRING
END TYPE
TYPE m
    v(1 TO 3) AS INTEGER
END TYPE
CONST k = 5
DIM i AS INTEGER, c AS STRING * 1, v AS STRING, bt AS _BIT, ba(7) AS _BIT, sa(1 TO 2) AS STRING, r AS t, w AS m
PUT #1, , "lit"
PUT #1, , 7
PUT #1, , i + 1
GET #1, , "lit"
GET #1, , bt
GET #1, ,
GET
GET #1, 1, c, 2
GET #, , c
GET "f", , c
GET #1, "a", c
PUT #1, , MID$(v, 1, 1)
PUT #1, , k
PUT #1, , ba()
GET (i), 1, c
PUT #1, 1, sa()
PUT #1, 1, r
PUT #1, 1, w.v()
GET #1, , LEN(v)
PUT
PUT #1, 1, c, 2
LOCK #1, TO 4
LOCK
LOCK #1, 2 TO
LOCK #1 TO 4
LOCK #1, 2, 4
LOCK "a"
UNLOCK
UNLOCK #1, v
UNLOCK #1, 2 TO v
' A FUNCTION's name with empty parentheses is no array (v30_x171, x172).
GET #1, , fn1()
PUT #1, , fn1()
FUNCTION fn1
    fn1 = 1
END FUNCTION
