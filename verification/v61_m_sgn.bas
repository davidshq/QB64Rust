$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SGN.
TYPE t
    sgn AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.sgn = 4: o.other = 2
o.sgn = o.sgn + o.other
PRINT o.sgn; o.other
SYSTEM
