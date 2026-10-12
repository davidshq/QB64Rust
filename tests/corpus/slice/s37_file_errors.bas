$CONSOLE:ONLY
' Runtime errors of the file statements under a handler (spec language/file-io, language/builtin-statements "Runtime
' errors"): 52 for a number that is not open, 53 for a missing file, 54 for the wrong mode, 55 for a number that is
' open, 62 past the end, 6 for a value outside the target's range, 63 for a bad position, 75 and 76 from the file
' system, 5 for a raising argument. A statement with several items stops at the first that raises; what was written
' or read before it stays. Files are s37_*.tmp and removed again.
ON ERROR GOTO handler
DIM f AS STRING, a AS STRING, c AS STRING, n AS LONG, b AS _BYTE, ub AS _UNSIGNED _BYTE
DIM arr(3) AS LONG, sa(3) AS STRING, retry AS LONG
f = "s37_errors.tmp"
PRINT "not open"
PRINT #3, "x"
WRITE #3, "x"
INPUT #3, a
LINE INPUT #3, a
SEEK 3, 1
PRINT EOF(3); LOF(3); LOC(3); SEEK(3)
PRINT "file numbers 0 and -1"
OPEN f FOR OUTPUT AS #0
OPEN f FOR OUTPUT AS #-1
PRINT #0, "x"
PRINT EOF(0)
PRINT "CLOSE of numbers that are not open is no error"
CLOSE #5: CLOSE 0, -1, 300
CLOSE
PRINT "a missing file"
OPEN "s37_none.tmp" FOR INPUT AS #1
OPEN "" FOR INPUT AS #1
PRINT "a number that is open"
OPEN f FOR OUTPUT AS #1
OPEN f FOR OUTPUT AS #1
PRINT "the wrong mode"
INPUT #1, a
LINE INPUT #1, a
PRINT #1, "one"
PRINT #1, "200,7"
PRINT #1, "-1,8"
PRINT #1, "31,32,33"
PRINT #1, "s1,s2,s3"
PRINT #1, "ra"; CHR$(-1); "rb"
PRINT #1, "after the raising item"
WRITE #1, "wa", CHR$(-1), "wb"
WRITE #1, "after the raising item"
CLOSE #1
OPEN f FOR INPUT AS #1
PRINT #1, "x"
WRITE #1, "x"
PRINT "the old form with a bad mode letter"
OPEN "X", #2, f
OPEN "", #2, f
PRINT "raising arguments"
OPEN CHR$(-1) FOR INPUT AS #2
OPEN f FOR INPUT AS ASC("")
CLOSE ASC("")
PRINT #ASC(""), "x"
INPUT #ASC(""), a
SEEK 1, ASC("")
PRINT EOF(ASC(""))
PRINT "a bad position"
SEEK 1, 0
SEEK 1, -5
PRINT "reading"
LINE INPUT #1, a: PRINT "["; a; "]"
PRINT "200 into a _BYTE: 0 is stored and the next target is not read"
b = 1: n = 1
INPUT #1, b, n
PRINT b; n
INPUT #1, a: PRINT "the next field is ["; a; "]"
PRINT "-1 into an unsigned type"
ub = 1
INPUT #1, ub, n
PRINT ub; n
LINE INPUT #1, a: PRINT "the rest of the line is ["; a; "]"
PRINT "a target with a bad index reads nothing and ends the statement"
n = 0: b = 0
INPUT #1, n, arr(9), b
PRINT n; b
LINE INPUT #1, a: PRINT "the rest of the line is ["; a; "]"
a = "": c = ""
INPUT #1, a, sa(9), c
PRINT "["; a; "]["; c; "]["; sa(0); "]"
LINE INPUT #1, a: PRINT "the rest of the line is ["; a; "]"
PRINT "what the raising items left in the file"
LINE INPUT #1, a: PRINT "["; a; "]"
LINE INPUT #1, a: PRINT "["; a; "]"
PRINT "past the end"; EOF(1)
a = "kept": n = 5
LINE INPUT #1, a: PRINT "["; a; "]"
INPUT #1, a, c
INPUT #1, n: PRINT n
CLOSE
PRINT "RESUME runs the OPEN again, after the handler made the file"
retry = 1
OPEN "s37_made.tmp" FOR INPUT AS #1
PRINT "opened; the handler ran"; retry - 1; "time"
retry = 0
CLOSE
KILL "s37_made.tmp"
PRINT "the file system"
OPEN f FOR INPUT AS #1
KILL f
NAME f AS "s37_other.tmp"
CLOSE
MKDIR "s37_dir"
OPEN "s37_dir/a.tmp" FOR OUTPUT AS #1: CLOSE
RMDIR "s37_dir"
KILL "s37_dir"
KILL "s37_dir/*.tmp"
KILL "s37_dir/*.tmp"
RMDIR "s37_dir"
KILL f
PRINT "in a SUB"
insub
SYSTEM
handler:
PRINT "  error"; ERR
IF ERR = 53 AND retry = 1 THEN
    retry = 2
    OPEN "s37_made.tmp" FOR OUTPUT AS #9: CLOSE #9
    RESUME
END IF
RESUME NEXT

SUB insub
    DIM t AS STRING
    PRINT #7, "x"
    PRINT "  after PRINT # in the SUB"
    LINE INPUT #7, t
    CLOSE #7
    PRINT "  end of the SUB"
END SUB
