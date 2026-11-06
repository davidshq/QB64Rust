' TEST: warnings
' `used-once` (spec `compiler/lints`, "Name used once"). Reported: `totl` (a misspelled read), `x` and `x$` (two
' variables, each named once), `grid` (an array used without `DIM`, named once), and `locl` in the SUB.
$CONSOLE:ONLY
total = 5
PRINT totl
PRINT total
DIM declared AS INTEGER
x = 1
PRINT x$
FOR i = 1 TO 3: PRINT "*": NEXT
INPUT "Press Enter", dummy$
LINE INPUT "Again", other$
READ skipped
DATA 1
grid(2) = 1
Work
SUB Work
    count = 1
    PRINT count
    PRINT locl
    FOR k = 1 TO 2
    NEXT
END SUB
