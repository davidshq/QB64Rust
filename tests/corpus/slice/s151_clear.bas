$CONSOLE:ONLY
' Slice (m4-builtins-param-types task 4.2): what CLEAR resets (verification/v49_e): variables of each kind, STATIC
' and SHARED ones, static arrays cleared and dynamic ones released, the DATA position, open files; CONST kept.
' CALL CLEAR and CLEAR with arguments; RETURN after CLEAR in a GOSUB is error 3.
TYPE ut
    s AS STRING
    n AS LONG
    f AS STRING * 3
END TYPE
COMMON SHARED cs AS LONG
DIM SHARED sh AS LONG, shs AS STRING
DIM a AS INTEGER, s AS STRING, fs AS STRING * 4, u AS ut, d AS DOUBLE, q AS _UNSIGNED _INTEGER64
DIM st(3) AS LONG, sts(2) AS STRING, stu(1) AS ut, b(1 TO 2) AS _BYTE
REDIM dy(4) AS LONG, dys(2) AS STRING
CONST K = 7
a = 5: s = "x": fs = "abcd": u.s = "q": u.n = 3: u.f = "zz": cs = 1: sh = 2: shs = "y": d = 1.5: q = 9
st(1) = 9: sts(1) = "w": stu(1).s = "v": stu(1).n = 8: dy(2) = 4: dys(1) = "d": b(2) = 3: implicit = 6
counter
counter
READ r: PRINT "read"; r
OPEN "s151.tmp" FOR OUTPUT AS #1
ON ERROR GOTO h
CLEAR
PRINT a; LEN(s); LEN(fs); ASC(fs); LEN(u.s); u.n; ASC(u.f); cs; sh; LEN(shs); d; q; K
PRINT st(1); LEN(sts(1)); LEN(stu(1).s); stu(1).n; b(2); UBOUND(st); implicit
PRINT "freefile"; FREEFILE
READ r: PRINT "read"; r
counter
ON ERROR GOTO h
PRINT "dynamic:"; UBOUND(dy)
PRINT "strings:"; UBOUND(dys)
REDIM dy(2) AS LONG: PRINT "again:"; UBOUND(dy); dy(1)
KILL "s151.tmp"
z = 1: CALL CLEAR: PRINT "z"; z
z = 2: CLEAR z + 1, , 100: PRINT "z"; z
z = 3: CLEAR , 2: PRINT "z"; z
ON ERROR GOTO h
GOSUB g
PRINT "done"
SYSTEM
g:
CLEAR
ON ERROR GOTO h
RETURN
PRINT "after return"
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
DATA 1,2
SUB counter
    STATIC c
    c = c + 1
    PRINT "c="; c
END SUB
