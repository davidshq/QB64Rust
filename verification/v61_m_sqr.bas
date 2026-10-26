$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SQR.
TYPE t
    sqr AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.sqr = 4: o.other = 2
o.sqr = o.sqr + o.other
PRINT o.sqr; o.other
SYSTEM
