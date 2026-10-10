' TEST: ir
$CONSOLE:ONLY
' Files in the IR (m2-builtin-statements tasks 3.3-3.6, design D2): OPEN names its table entry (form 1 or 2) with
' the name, the words chosen, the number and the absent slots, never sub_open or a mode number; CLOSE one value per
' number; SEEK two values; PRINT and WRITE with the file they go to and their items; INPUT and LINE INPUT with their
' source and target places. Each may raise
DIM f AS STRING, a AS STRING, n AS LONG, d AS DOUBLE
DIM arr(3) AS LONG
OPEN f FOR OUTPUT AS #1
OPEN f FOR INPUT ACCESS READ SHARED AS n LEN = 8
OPEN "A", #2, f
OPEN "R", 3, f, 16
CLOSE
CLOSE #1, n
SEEK 1, d
PRINT #1, "x"; n, d
PRINT #1, , "y";
PRINT #n,
WRITE #1, n, a
WRITE n, a,
INPUT #1, a, n, arr(2)
LINE INPUT #n, a
