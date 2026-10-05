$CONSOLE:ONLY
' Verification (m2-parser-breadth, M2): letter case and inner quotes of unquoted items are kept as written?
data MixedCase, a"b, x
DATA "END"
DO
    READ s$
    IF s$ = "END" THEN EXIT DO
    PRINT "["; s$; "]"
LOOP
SYSTEM
