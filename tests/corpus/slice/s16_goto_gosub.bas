$CONSOLE:ONLY
' GOTO, GOSUB, RETURN and labels (spec language/control-flow, "GOTO", "GOSUB and RETURN", "Labels per body"):
' jumps into a block (IF branch, FOR, WHILE, DO bodies) and out of one, a FOR entered by GOTO before it ever ran
' (its hidden limit and step are 0; capped), GOSUB nested and from inside a block, RETURN label, GOSUB inside a
' SUB, the same label name in main and in two SUBs, a label inside a block of a SUB, RETURN with nothing pending in
' main and in a SUB (error 3), GOSUB from a handler, RETURN in a SUB while a main GOSUB is pending (error 3, the
' entry is consumed).
ON ERROR GOTO h
PRINT "GOTO into an IF branch whose condition is false"
x = 0
GOTO inif
IF x = 1 THEN
    PRINT "  then 1"
    inif:
    PRINT "  then 2"
ELSE
    PRINT "  else"
END IF
PRINT "  after END IF"
PRINT "GOTO into a FOR that never ran (capped)"
n = 0
GOTO infor
FOR i = 1 TO 3
    PRINT "  body 1"; i
    infor:
    n = n + 1: PRINT "  body 2"; i
    IF n >= 3 THEN EXIT FOR
NEXT
PRINT "  after"; i
PRINT "GOTO out of a FOR, then the FOR again"
FOR i = 1 TO 5
    IF i = 2 THEN GOTO outfor
    PRINT "  body"; i
NEXT
outfor:
PRINT "  out at"; i
FOR i = 1 TO 2
    PRINT "  second run"; i
NEXT
PRINT "GOTO into a WHILE body"
n = 0
GOTO inwhile
WHILE n < 3
    PRINT "  top"; n
    inwhile:
    n = n + 1: PRINT "  in"; n
    IF n > 6 THEN EXIT WHILE
WEND
PRINT "  after"; n
PRINT "GOTO into a DO ... LOOP UNTIL body"
n = 0
GOTO indo
DO
    PRINT "  top"; n
    indo:
    n = n + 1: PRINT "  in"; n
    IF n > 6 THEN EXIT DO
LOOP UNTIL n >= 3
PRINT "  after"; n
PRINT "a backward GOTO as a loop"
n = 0
lab:
n = n + 1
IF n < 3 THEN GOTO lab
PRINT "  n"; n
PRINT "GOSUB"
GOSUB g1
PRINT "  back from g1"
FOR i = 1 TO 2
    GOSUB g1
NEXT
PRINT "GOSUB nested"
GOSUB outer
PRINT "  back from outer"
PRINT "RETURN label"
GOSUB g2
PRINT "  not printed"
back2:
PRINT "  at back2"
GOSUB g1
PRINT "  GOSUB after RETURN label works"
PRINT "SUBs with their own labels"
s1
s2
PRINT "RETURN with nothing pending"
RETURN
PRINT "  after RETURN"
sret
PRINT "  back from sret"
PRINT "GOSUB from a handler"
ERROR 9
PRINT "  after ERROR 9"
PRINT "RETURN in a SUB while a main GOSUB is pending (ends the program)"
GOSUB g3
PRINT "  not printed: g3's RETURN has nothing to return to"
SYSTEM

g1:
PRINT "  in g1"
RETURN

outer:
PRINT "  in outer"
GOSUB inner
PRINT "  outer again"
RETURN

inner:
PRINT "  in inner"
RETURN

g2:
PRINT "  in g2"
RETURN back2

g3:
PRINT "  in g3"
sret
PRINT "  g3 after sret"
RETURN
PRINT "  after g3's RETURN"
SYSTEM

hsub:
PRINT "  in hsub"
RETURN

h:
PRINT "  handler"; ERR
IF ERR = 9 THEN GOSUB hsub
RESUME NEXT

SUB s1
    PRINT "  in s1"
    GOSUB lab
    PRINT "  s1 done"
    EXIT SUB
    lab:
    PRINT "  lab in s1"
    RETURN
END SUB

SUB s2
    n = 0
    IF n = 0 THEN
        lab:
        n = n + 1
    END IF
    IF n < 3 THEN GOTO lab
    PRINT "  s2 n"; n
    GOSUB lab2
    PRINT "  s2 done"
    EXIT SUB
    lab2:
    PRINT "  lab2 in s2"
    RETURN
END SUB

SUB sret
    PRINT "  in sret"
    RETURN
    PRINT "  sret after RETURN"
END SUB
