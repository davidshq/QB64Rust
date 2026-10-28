$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named MOD.
TYPE t
    mod AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.mod = 4: o.other = 2
o.mod = o.mod + o.other
PRINT o.mod; o.other
SYSTEM
