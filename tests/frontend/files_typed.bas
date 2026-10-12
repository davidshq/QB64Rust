' TEST: typed
$CONSOLE:ONLY
' Files in the typed tree (m2-builtin-statements tasks 3.3-3.6; measured in verification\v22_b_*): OPEN in both
' forms with its words as slots, CLOSE with any number of file numbers, SEEK with an _INTEGER64 position; the file
' functions: EOF, FREEFILE, _FILEEXISTS, _DIREXISTS LONG, LOF, LOC, SEEK held _INTEGER64 and believed LONG, _CWD$ a
' string, FREEFILE and _CWD$ bare; a float file number converted as a LONG slot; PRINT # and WRITE items; INPUT #
' and LINE INPUT # targets (an implicit variable is created)
DIM f AS STRING, a AS STRING, n AS LONG, d AS DOUBLE, q AS _INTEGER64
DIM fx AS STRING * 4, arr(3) AS LONG
TYPE pt
    x AS LONG
    t AS STRING * 3
END TYPE
DIM p AS pt
OPEN f FOR OUTPUT AS #1
OPEN f FOR RANDOM ACCESS READ WRITE LOCK WRITE AS n LEN = 4
OPEN f AS d
OPEN "O", #1, f
OPEN a, n, f, 10
CLOSE
CLOSE #1, n, 2.5
SEEK 1, d
SEEK #n, q
n = EOF(1) + LOF(n) + LOC(d) + SEEK(q) + FREEFILE
n = _FILEEXISTS(f) + _DIREXISTS("d")
a = _CWD$
q = LOF(1) * 1000000000
PRINT #1, "x"; n, d
PRINT #d,
PRINT #1, a;
WRITE #1, n, a, d
WRITE n, a, fx,
WRITE
INPUT #1, a, n, fresh, arr(2), p.x, p.t, fx
LINE INPUT #n, a
LINE INPUT #1, fx
