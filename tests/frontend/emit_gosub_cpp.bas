' TEST: cpp
$CONSOLE:ONLY
' GOSUB and RETURN emitted (m2-control-flow-slice task 7.2, design D9): GOSUB pushes a program-wide return id,
' jumps, and has `RETURN_<G>:;` inside its own statement; RETURN includes retK.txt of its body (ret0.txt for main,
' case 0 returns from QBMAIN; ret1.txt for the SUB, case 0 is error 3), which has one case per GOSUB of that body
' and raises error 3 for anything else; RETURN label decrements only when a GOSUB is pending (DIVERGENCES.md
' D-003). The same label name in main and in the SUB is fine (C++ labels are per function); the SUB's FOR has its
' temporaries in data1.txt, new on every call
GOSUB again
GOSUB back
show 2
SYSTEM
again:
PRINT "main"
RETURN
back:
RETURN done
done:
PRINT "done"
SUB show (n)
    GOSUB again
    EXIT SUB
    again:
    FOR i = 1 TO n
        PRINT i
    NEXT
    RETURN
END SUB
