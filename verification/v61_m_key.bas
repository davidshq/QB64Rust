$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named KEY.
TYPE t
    key AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.key = 4: o.other = 2
o.key = o.key + o.other
PRINT o.key; o.other
SYSTEM
