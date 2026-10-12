$CONSOLE:ONLY
' DO and WHILE (spec language/control-flow, "DO and WHILE loops", "EXIT from loops"): WHILE, DO WHILE and DO UNTIL
' test before each pass, LOOP WHILE and LOOP UNTIL after it; DO ... LOOP left by EXIT DO; EXIT DO and EXIT WHILE
' from inside other blocks; EXIT leaves only the innermost block of its kind; loops in a SUB.
PRINT "WHILE"
n = 0
WHILE n < 3
    n = n + 1: PRINT n;
WEND
PRINT
WHILE n < 0
    PRINT "  never"
WEND
PRINT "DO WHILE"
n = 0
DO WHILE n < 3
    n = n + 1: PRINT n;
LOOP
PRINT
PRINT "DO UNTIL"
n = 0
DO UNTIL n >= 3
    n = n + 1: PRINT n;
LOOP
PRINT
DO UNTIL n > 0
    PRINT "  never"
LOOP
PRINT "LOOP WHILE"
n = 5
DO
    PRINT n;: n = n + 1
LOOP WHILE n < 3
PRINT
n = 0
DO
    n = n + 1: PRINT n;
LOOP WHILE n < 3
PRINT
PRINT "LOOP UNTIL"
n = 5: DO: PRINT n;: n = n + 1: LOOP UNTIL n > 3: PRINT
n = 0
DO
    n = n + 1: PRINT n;
LOOP UNTIL n = 3
PRINT
PRINT "DO ... LOOP with EXIT DO"
n = 0
DO
    n = n + 1
    IF n = 4 THEN EXIT DO
    PRINT n;
LOOP
PRINT
PRINT "  after"; n
PRINT "EXIT WHILE from inside a block IF"
w = 0
WHILE 1
    w = w + 1
    IF w = 4 THEN
        PRINT "  leaving at"; w
        EXIT WHILE
    END IF
WEND
PRINT "  after"; w
PRINT "EXIT DO leaves the inner DO only"
o = 0
DO WHILE o < 2
    o = o + 1
    i = 0
    DO
        i = i + 1
        IF i = 3 THEN EXIT DO
    LOOP
    PRINT "  outer"; o; "inner"; i
LOOP
PRINT "EXIT DO from inside a WHILE inside a DO"
o = 0
DO
    o = o + 1
    w = 0
    WHILE w < 5
        w = w + 1
        IF o = 2 AND w = 3 THEN EXIT DO
    WEND
    PRINT "  pass"; o; w
LOOP
PRINT "  after"; o; w
PRINT "string condition through a comparison"
s$ = ""
DO WHILE s$ <> "aaa"
    s$ = s$ + "a"
LOOP
PRINT "  "; s$
WHILE s$ < "aaaaa"
    s$ = s$ + "a"
WEND
PRINT "  "; s$
PRINT "loops in a SUB"
countdown 3
countdown 0
SYSTEM

SUB countdown (m)
    PRINT " ";
    DO WHILE m > 0
        PRINT m;
        m = m - 1
    LOOP
    DO
        PRINT "done";
    LOOP UNTIL 1
    PRINT
END SUB
