$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DOUBLE.
TYPE t
    double AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.double = 4: o.other = 2
o.double = o.double + o.other
PRINT o.double; o.other
SYSTEM
