' TEST: check-fail
$CONSOLE:ONLY
' Forms near those of m4-memory-records that the old compiler rejects, one a line (design D11; verification\v21_x54,
' x55, x57, v51_x75, v56_x15-x17, x19-x22): STRING * n with a variable, a name never assigned or a string
' constant ("Number/Constant expected after *"), and a name f$5 as a SUB's ("Type symbols after a SUB name are
' invalid"), a CONST's ("Invalid constant type") or a member's ("Invalid name"). Then a mark: LEN of a TYPE whose
' member array bounds name a CONST, above its block, before pass 2 has read them (design D7)
CONST n$ = "ab"
v = 2
DIM s1 AS STRING * v
DIM s2 AS STRING * never
DIM s3 AS STRING * n$
CONST fs$5 = "a"
TYPE bad
    fs$5 AS STRING
END TYPE
DIM early AS late
PRINT LEN(early)
CONST top = 3
TYPE late
    w(top) AS LONG
END TYPE
SYSTEM

SUB fs$5
END SUB
