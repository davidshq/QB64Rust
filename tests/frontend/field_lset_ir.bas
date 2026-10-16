' TEST: ir
$CONSOLE:ONLY
' FIELD, LSET and RSET in the IR (m3-console-files task 6.1, design D3; verification\v30_j_field,
' v30_k_field_errors, v30_l_lset_rset, v30_x100-x170): FIELD is the file number and a list of (string place,
' width), each a LONG; LSET and RSET are a string place of either kind and a string value. The record buffer and a
' string's link to its field are the runtime's: a field variable is an ordinary variable-length string here
TYPE t
    s AS STRING * 6
    v AS STRING
    n AS LONG
END TYPE
DIM a AS STRING, b AS STRING, x AS STRING * 5, e(1 TO 3) AS STRING, r AS t, q(2) AS t, k AS LONG, s AS SINGLE
DIM d AS DOUBLE, fx(2) AS STRING * 4
' One and several parts; the number sign left out; a file number and a width that are expressions
FIELD #1, 3 AS a
FIELD 1, 3 AS a, 5 AS b
FIELD #k + 1, s AS e(k), d + 1 AS r.v
FIELD #d, 2 AS q(k).v
' A variable in parentheses, a name not seen before
FIELD #1, 4 AS (a)
FIELD #1, 4 AS fresh$
' LSET and RSET: variable-length and fixed-length strings, elements, members
LSET a = "xy"
RSET a = b
LSET x = "ab"
RSET e(k) = a + b
LSET r.s = "ab"
LSET r.v = x
RSET q(k).s = r.s
LSET fx(1) = "z"
LSET a = MID$(a, 2)
IF k THEN LSET a = "x" ELSE RSET b = "y"
' GET and PUT with no variable read and write the record the fields name
GET #1, 2
PUT #1
SYSTEM
