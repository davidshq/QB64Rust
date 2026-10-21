$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): what CLEAR resets and what it keeps.
TYPE ut
    s AS STRING
    n AS LONG
    f AS STRING * 3
END TYPE
COMMON SHARED cs AS LONG
DIM SHARED sh AS LONG, shs AS STRING
DIM a AS INTEGER, s AS STRING, fs AS STRING * 4, u AS ut, d AS DOUBLE
DIM st(3) AS LONG, sts(2) AS STRING, stu(1) AS ut
REDIM dy(4) AS LONG
CONST K = 7
a = 5: s = "x": fs = "abcd": u.s = "q": u.n = 3: u.f = "zz": cs = 1: sh = 2: shs = "y": d = 1.5
st(1) = 9: sts(1) = "w": stu(1).s = "v": stu(1).n = 8: dy(2) = 4: implicit = 6: implicit$ = "i"
counter
counter
READ r: PRINT "read"; r
OPEN "v49e.tmp" FOR OUTPUT AS #1
ON ERROR GOTO h
CLEAR
PRINT a; LEN(s); "["; fs; "]"; LEN(fs); LEN(u.s); u.n; "["; u.f; "]"; cs; sh; LEN(shs); d; K
PRINT st(1); LEN(sts(1)); LEN(stu(1).s); stu(1).n; UBOUND(st); implicit; LEN(implicit$)
PRINT "freefile"; FREEFILE
READ r: PRINT "read"; r
counter
ON ERROR GOTO h
PRINT "dynamic:"; UBOUND(dy)
PRINT "element:"; dy(2)
REDIM dy(2) AS LONG: PRINT "again:"; UBOUND(dy); dy(1)
KILL "v49e.tmp"
z = 1: CLEAR , , 100: PRINT "z"; z
z = 2: CLEAR 1: PRINT "z"; z
z = 3: CLEAR 1, 2, 3: PRINT "z"; z
z = 4: CLEAR , 2: PRINT "z"; z
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
