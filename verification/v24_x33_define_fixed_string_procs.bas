$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a parameter and a FUNCTION result typed STRING * 5 by _DEFINE.
_DEFINE a AS STRING * 5
show "abcdefgh"
PRINT "["; a2; "]"
SYSTEM
SUB show (a1)
    PRINT "["; a1; "]"; LEN(a1)
END SUB
FUNCTION a2
    a2 = "xy"
END FUNCTION
