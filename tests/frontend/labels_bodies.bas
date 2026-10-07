' TEST: typed
$CONSOLE:ONLY
' The same label name in main and in two SUBs is three labels; each jump names a label of its own body, also one
' further down; ON ERROR GOTO in a SUB names a label of main (m2-control-flow-slice task 5.1, design D5)
GOTO a
a:
GOSUB g
t1
t2
SYSTEM
g:
RETURN
h:
RESUME NEXT
SUB t1
    GOTO a
    a:
    GOSUB g
    EXIT SUB
    g:
    RETURN
END SUB
SUB t2
    ON ERROR GOTO h
    a: PRINT "t2"
    GOTO a
END SUB
