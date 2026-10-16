$CONSOLE:ONLY
' Slice (m3-console-files, task 7.1): CHAIN, compiled and never run. A program prints a line and leaves; below its
' SYSTEM stands CHAIN in each form, as code that is compiled, linked and not reached: when it runs, libqb starts
' the compiler beside the program through cmd. The call each form compiles to is compared by tests\callsite.
DIM f AS STRING, x AS STRING * 8, n AS LONG
f = "next"
PRINT "CHAIN: compiled"
IF LEN(f) = 0 THEN CHAIN f
SYSTEM

CHAIN "next.bas"
CHAIN f
CHAIN x
CHAIN f + ".bas"
CHAIN ("next")
CALL CHAIN(f)
IF n THEN CHAIN f ELSE CHAIN "other"
