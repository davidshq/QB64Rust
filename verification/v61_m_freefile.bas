$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named FREEFILE.
TYPE t
    freefile AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.freefile = 4: o.other = 2
o.freefile = o.freefile + o.other
PRINT o.freefile; o.other
SYSTEM
