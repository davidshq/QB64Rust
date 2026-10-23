$CONSOLE:ONLY
' Verification (m4-builtins-param-types, task 1.1): a DECLARE SUB below the TYPE block for a SUB above it.
PRINT 1
SYSTEM
SUB use (p AS late)
END SUB
TYPE late
    k AS LONG
END TYPE
DECLARE SUB use (p AS late)
