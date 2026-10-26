$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named BASE.
TYPE t
    base AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.base = 4: o.other = 2
o.base = o.base + o.other
PRINT o.base; o.other
SYSTEM
