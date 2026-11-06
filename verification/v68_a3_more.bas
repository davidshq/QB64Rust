$CONSOLE:ONLY
' v68 "Unused variable": more declarations, each never named again unless the comment says so.
COMMON plaincommon AS INTEGER
DIM setonly AS INTEGER: setonly = 1
REDIM redimonly(5) AS LONG
DIM a1 AS INTEGER, a2 AS INTEGER: PRINT a2
DIM SHARED sharedsub AS LONG
Stat
PRINT Calls(1)
SYSTEM

SUB Stat
    STATIC st AS INTEGER
    SHARED plaincommon
    DIM sharedsub AS LONG
END SUB

FUNCTION Calls (k)
    Calls = k
    IF k < 1 THEN PRINT Calls(k + 1)
END FUNCTION
