' TEST: warnings
' QB64pe's warnings (spec `compiler/lints`, "QB64pe's warnings"; measured, `verification\v68_*`): an unused DIM and
' parameter, an empty SELECT CASE at its END SELECT, a repeated CONST, and RUN with $DEBUG.
$CONSOLE:ONLY
$DEBUG
DIM spare AS LONG
CONST k = 3
CONST k = 3
n = k
SELECT CASE n
END SELECT
Show n, "x"
IF n = 0 THEN RUN
SUB Show (v, extra AS STRING)
    PRINT v
END SUB
