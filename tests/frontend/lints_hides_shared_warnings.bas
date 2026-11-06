' TEST: warnings
' `hides-shared` (spec `compiler/lints`, "Local hiding a shared variable"): reported for the `DIM` in `Local` and
' the `REDIM` in `Again`. Quiet for `SHARED score`, for a local of another type (`Other`), and for a parameter of
' the name (`Param`).
$CONSOLE:ONLY
DIM SHARED score AS LONG
score = 1
Local
Again
UsesShared
Other
Param 3
SUB Local
    DIM score AS LONG
    score = 2
    PRINT score
END SUB
SUB Again
    REDIM score AS LONG
    PRINT score
END SUB
SUB UsesShared
    SHARED score
    PRINT score
END SUB
SUB Other
    DIM score AS STRING
    score = "x"
    PRINT score
END SUB
SUB Param (score AS LONG)
    PRINT score
END SUB
