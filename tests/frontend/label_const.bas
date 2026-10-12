' TEST: check-ok
$CONSOLE:ONLY
' A label and a CONST on one line define both (DIVERGENCES.md D-008, m2-numeric-types design D8; the old compiler
' fails to compile it), with a long and a short label, in the main module and in a SUB.
lbl1: CONST k = 4
x: CONST y = 5
PRINT k; y: GOTO done
done:
s
SYSTEM

SUB s
    here: CONST z = 6
    PRINT z
END SUB
