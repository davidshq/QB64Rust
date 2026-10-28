$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LOF.
TYPE t
    lof AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.lof = 4: o.other = 2
o.lof = o.lof + o.other
PRINT o.lof; o.other
SYSTEM
