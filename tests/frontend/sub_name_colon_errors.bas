' TEST: check-fail
$CONSOLE:ONLY
' Labels with the name of a SUB, where the old compiler reports an error (verification\v28_x71, x72, x74): a jump
' to a SUB's name that no line of the body has with a colon ("Label 'one' not defined"), also when another body
' has such a line (two); the name twice in a body with a jump to it ("Duplicate label", three). The last case
' is this compiler's own: a line that could not be read has the word in it, so whether `five:` is a call or a
' label is not known, and it keeps its mark
DIM n AS INTEGER
GOTO one
two:
three:
three:
IF n THEN GOTO three
five:
GOSUB five five
' Every other statement that names a label makes its word one (x82-x89): each name stands twice below, which is
' a duplicate label only when the line is a label. `IF n THEN t8` is no jump (x91): t8 twice is two calls
t1:
t1:
RESUME t1
t2:
t2:
RETURN t2
t3:
t3:
ON n GOTO five, t3
t4:
t4:
ON n GOSUB t4
t5:
t5:
ON KEY(1) GOSUB t5
t6:
t6:
ON TIMER(1) GOSUB t6
t7:
t7:
ON STRIG(0) GOSUB t7
t9:
t9:
RESTORE t9
t10:
t10:
ON ERROR GOTO t10
t8:
t8:
IF n THEN t8
SYSTEM

SUB one
END SUB

SUB two
END SUB

SUB three
END SUB

SUB four
    GOTO two
END SUB

SUB five
END SUB

SUB t1: END SUB
SUB t2: END SUB
SUB t3: END SUB
SUB t4: END SUB
SUB t5: END SUB
SUB t6: END SUB
SUB t7: END SUB
SUB t8: END SUB
SUB t9: END SUB
SUB t10: END SUB
