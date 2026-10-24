$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON over a program with every kind of statement and block, a
' label, a line number, a joined line, a single-line IF, a GOSUB, a SUB, a FUNCTION, DATA, a TYPE, an included
' file and a handler (DEF FN is a stub of the old compiler); it prints and ends. Its C++ (-z) places the old
' compiler's error_track_line calls (task 2.2, design D6).
$ERRORLOCATION:ON
TYPE pt
    x AS LONG
END TYPE
DIM p AS pt, a(3) AS LONG
CONST K = 2
DATA 1, 2
ON ERROR GOTO handler
READ a(1), a(2)
p.x = K
10 PRINT "start"; a(1); a(2); p.x
IF p.x = 2 THEN PRINT "one line if" ELSE PRINT "no"
IF p.x = 2 THEN
    PRINT "block if"
ELSEIF p.x = 3 THEN
    PRINT "elseif"
ELSE
    PRINT "else"
END IF
FOR i = 1 TO 2
    PRINT "for"; i
NEXT
DO
    i = i - 1
LOOP WHILE i > 0
WHILE i < 2
    i = i + 1
WEND
SELECT CASE i
    CASE 2
        PRINT "case 2"
    CASE ELSE
        PRINT "case else"
END SELECT
GOSUB sub1
PRINT "a"; _
    "b"
s "x"
PRINT f(3)
'$INCLUDE:'v58_inc/e_on.bi'
ERROR 5
PRINT "after"
label1:
PRINT "end"
SYSTEM
sub1:
PRINT "gosub"
RETURN
handler:
PRINT "error"; ERR
RESUME NEXT

SUB s (t$)
    PRINT "sub "; t$
END SUB

FUNCTION f (n)
    f = n + 1
END FUNCTION
