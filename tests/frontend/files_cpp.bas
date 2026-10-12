' TEST: cpp
$CONSOLE:ONLY
' Files as C++ (m2-builtin-statements tasks 3.3-3.6, design D5; the old compiler's lines are in study\00 section 5):
' sub_open(name,mode,access,lock,number,length,passed) with NULL for a word left out, sub_open_gwbasic, sub_close per
' number or sub_close(NULL,0), sub_seek; PRINT #: tmp_fileno, one sub_file_print(tmp_fileno,text,extraspace,tab,
' newline) per item with the pending-error test after each; WRITE: the item text built with qbs_ltrim and quotes;
' INPUT #: func_file_input_float with the target's type code, the 64-bit readers, sub_file_input_string, each stored
' by its place's rule; LINE INPUT #: sub_file_line_input_string
DIM f AS STRING, a AS STRING, n AS LONG, d AS DOUBLE, q AS _INTEGER64, b AS _BYTE, s AS SINGLE
DIM bt AS _BIT * 5, fx AS STRING * 4, arr(3) AS LONG, sa(3) AS STRING
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
CLOSE #1, n, d
SEEK n, d
n = EOF(1) + LOF(n) + LOC(d) + SEEK(q) + FREEFILE + _FILEEXISTS(f) + _DIREXISTS(f)
a = _CWD$
PRINT #1, "x"; n, d
PRINT #n, , a; fx;
PRINT #1,
PRINT #1, ;
WRITE #1, n, a, d
WRITE #1,
WRITE n, a, fx
WRITE
WRITE #1, 1,
INPUT #1, a, b, n, q, s, d
INPUT #d, bt, fx, sa(1), arr(2), p.x, p.t
LINE INPUT #1, a
LINE INPUT #n, sa(2)
