' TEST: check-fail
$CONSOLE:ONLY
' File statements rejected (verification\v22_x21-x60): a string for a file number, a number for a file name; PRINT #
' items without a separator; INPUT # targets that are no variable (a literal, an expression, a CONST), no target, a
' whole TYPE variable; LINE INPUT # into a number or two targets; the file functions with a wrong argument or
' count. Left out and "not supported yet": a whole array as a target, a member of an element as a target, TAB in a
' PRINT #, PRINT # USING
CONST c = 5
DIM a AS STRING, b AS STRING, n AS LONG, arr(3) AS LONG
TYPE pt
    x AS LONG
END TYPE
DIM v AS pt, va(2) AS pt
OPEN "f" FOR INPUT AS "a"
OPEN 5 FOR INPUT AS #1
OPEN "f" FOR RANDOM AS #1 LEN = "a"
OPEN 1, #1, "f"
CLOSE "a"
SEEK "a", 1
SEEK 1, "a"
PRINT #"a", "x"
PRINT #1, "a" "b"
PRINT #1, "a" 1
WRITE #a, 1
INPUT #1, 5
INPUT #1, n + 1
INPUT #1, c
INPUT #1, v
INPUT #1, n, , a
LINE INPUT #1, n
LINE INPUT #1, a, b
LINE INPUT #1, "a"
INPUT #a, n
PRINT EOF("a")
PRINT EOF
PRINT FREEFILE(1)
PRINT FREEFILE()
PRINT _CWD$(1)
PRINT _FILEEXISTS(5)
PRINT SEEK(1, 2)
PRINT LOF()
INPUT #1, arr()
INPUT #1, va(1).x
PRINT #1, TAB(5); "t"
PRINT #1, USING "##"; n
WRITE , 1

FUNCTION fr&
    INPUT #1, fr&
END FUNCTION

FUNCTION fs$
    LINE INPUT #1, fs$
END FUNCTION
