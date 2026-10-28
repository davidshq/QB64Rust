$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CVI.
TYPE t
    cvi AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.cvi = 4: o.other = 2
o.cvi = o.cvi + o.other
PRINT o.cvi; o.other
SYSTEM
