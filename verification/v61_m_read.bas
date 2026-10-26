$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named READ.
TYPE t
    read AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.read = 4: o.other = 2
o.read = o.read + o.other
PRINT o.read; o.other
SYSTEM
