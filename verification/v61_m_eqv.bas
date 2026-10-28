$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named EQV.
TYPE t
    eqv AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.eqv = 4: o.other = 2
o.eqv = o.eqv + o.other
PRINT o.eqv; o.other
SYSTEM
