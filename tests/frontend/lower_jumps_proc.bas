' TEST: ir
$CONSOLE:ONLY
' GOTO, GOSUB and RETURN lowered (m2-control-flow-slice task 6.2, design D8), in main and in a SUB: each jump names
' a label of its own body (both bodies have a label `again`); RETURN label names a main label. A FOR in a SUB
' has temporaries of the SUB, new on every call (Storage::Temp of the SUB, listed with its variables)
GOSUB again
GOTO skip
again:
PRINT "sub"
RETURN
skip:
GOSUB back
show 2
END
back:
RETURN skip
SUB show (n)
    GOSUB again
    EXIT SUB
    again:
    FOR i = 1 TO n
        PRINT i
    NEXT
    RETURN
END SUB
