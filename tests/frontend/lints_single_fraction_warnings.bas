' TEST: warnings
' `single-fraction` (spec `compiler/lints`, "A SINGLE that holds a fraction"). Reported: `rate!` on line 16 and
' `rate` on line 17, under a `+`; `cut` on line 18, set to a negative fraction; and `share` in `Split`, a shared
' SINGLE set in the main module and by `VAL`. Not reported: `half!`, set to .5, which a SINGLE holds exactly;
' `d#`, a DOUBLE; the product on line 21, stored in a SINGLE; the product inside `CINT` on line 22, which is not
' looked into; `whole!`, set to 3; and in `Computed`, `p!`, which an assignment also computes, and `k!`, a counter.
$CONSOLE:ONLY
DIM SHARED share AS SINGLE
rate! = .3
half! = .5
d# = .3
cut! = -.7
whole! = 3
share = .1
r% = .3 * 95
r% = rate! * 95
r& = 1 + rate * 7
r% = cut! * 10
r% = half! * 95
r% = d# * 95
f! = rate! * 95
r% = CINT(rate! * 95)
r% = whole! * 5
PRINT r%; r&; f!
Split: Computed
SUB Split
    share = VAL("0.2")
    n% = share * 30
    PRINT n%
END SUB
SUB Computed
    p! = .1
    p! = p! * 2 + .05
    n% = p! * 30
    k! = .1
    FOR k! = .1 TO 1 STEP .1
        n% = k! * 30
    NEXT
    ' Reported: `q!`, set from a CONST, which the warning names with its value, and `w!`, set from it negated.
    CONST KNOB = .3
    q! = KNOB
    n% = q! * 30
    w! = -KNOB
    n% = w! * 30
    PRINT n%
END SUB
