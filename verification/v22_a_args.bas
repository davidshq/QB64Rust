$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Statement calls"): argument forms the old compiler accepts for the plain
' statements, to read the C++ of each (`qb64pe -z`): a literal, a variable, an expression, a fixed-length string, an
' element, a member-free form in parentheses, CALL syntax, and SEEK's numeric arguments of several types.
ON ERROR GOTO h
DIM s AS STRING
DIM fx AS STRING * 12
DIM a(2) AS STRING
DIM l AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64, sg AS SINGLE
s = "v22_a_none.tmp"
fx = "v22_a_none"
a(1) = "v22_a_none.tmp"
KILL "v22_a_none.tmp"
KILL s
KILL s + ".x"
KILL fx
KILL a(1)
KILL (s)
MKDIR s + "d"
RMDIR s + "d"
CHDIR "."
NAME s AS s + "2"
NAME "v22_a_n1" AS "v22_a_n2"
ENVIRON s
OPEN "v22_a_seek.tmp" FOR OUTPUT AS #1
PRINT #1, "0123456789"
l = 1: i = 3: d = 2.5: q = 4: sg = 3.5
SEEK 1, 2
SEEK #1, 2
SEEK l, i
SEEK 1, d
PRINT SEEK(1)
SEEK 1, sg
PRINT SEEK(1)
SEEK 1, q
PRINT SEEK(1)
SEEK 1.4, 2.5
PRINT SEEK(1)
SEEK 1, 3.5
PRINT SEEK(1)
SEEK 1, 0
PRINT "after SEEK 1, 0"
SEEK 1, -1
PRINT "after SEEK 1, -1"
SEEK 2, 1
PRINT "after SEEK 2, 1"
SEEK 0, 1
PRINT "after SEEK 0, 1"
CLOSE #1
KILL "v22_a_seek.tmp"
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
