$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PSET.
TYPE t
    pset AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.pset = 4: o.other = 2
o.pset = o.pset + o.other
PRINT o.pset; o.other
SYSTEM
