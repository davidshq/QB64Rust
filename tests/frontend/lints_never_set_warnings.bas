' TEST: warnings
' `never-set` (spec `compiler/lints`, "A local variable that is never set"). Reported, once each at its first
' read: `cash` in `Fortify`, which names the shared array `cash()` (Civil War Strategy's `ai_fortify`); `total`
' in `Sum`, an implicit local read twice; `title` in `Heading`, a string, so `""`; `count` in `Counter`, a
' `STATIC`; and `best` in the FUNCTION `Pick`. Not reported: `tally`, which an assignment sets; `n` in `Calls`,
' which the FUNCTION `Bump` sets through its parameter; `a` and `b`, which `SWAP` sets; `got`, which `Fill`
' sets; `reply`, which `INPUT` sets; `blank`, a record, and `pad`, a fixed-length string; `p`, a parameter;
' `once`, named once, which gets `used-once`; `unused`, declared and never named again, which gets "Unused
' variable"; and `outside` in the main module.
$CONSOLE:ONLY
TYPE Rec
    a AS INTEGER
END TYPE
DIM SHARED cash(1) AS INTEGER
cash(0) = 500
PRINT outside; outside
Fortify 0
Sum
Heading
Counter
PRINT Pick
Others 1
SUB Fortify (side)
    DIM cash AS INTEGER
    IF cash < 222 THEN PRINT "stop"; cash(side)
END SUB
SUB Sum
    PRINT total
    PRINT total + 1
END SUB
SUB Heading
    DIM title AS STRING
    PRINT title
END SUB
SUB Counter
    STATIC count AS LONG
    PRINT count
END SUB
FUNCTION Pick
    DIM best AS INTEGER
    Pick = best
END FUNCTION
SUB Others (p)
    DIM tally AS INTEGER
    tally = 3
    PRINT tally
    DIM n AS INTEGER
    PRINT Bump(n); n
    DIM a AS INTEGER, b AS INTEGER
    SWAP a, b
    PRINT a; b
    DIM got AS INTEGER
    Fill got
    PRINT got
    DIM reply AS STRING
    INPUT reply
    PRINT reply
    DIM blank AS Rec
    PRINT blank.a
    DIM pad AS STRING * 4
    PRINT pad
    PRINT p
    PRINT once
    DIM unused AS INTEGER
    ' Not reported: `poked`, whose memory `VARPTR` takes, and `later`, which only a skipped `$IF` branch sets.
    DIM poked AS INTEGER
    PRINT VARPTR(poked); poked
    DIM later AS INTEGER
    $LET HAVE = 0
    $IF HAVE = 1 THEN
        later = 1
    $END IF
    PRINT later
END SUB
FUNCTION Bump (x AS INTEGER)
    x = x + 1
    Bump = x
END FUNCTION
SUB Fill (v AS INTEGER)
    v = 7
END SUB
