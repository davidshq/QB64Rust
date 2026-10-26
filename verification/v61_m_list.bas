$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LIST.
TYPE t
    list AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.list = 4: o.other = 2
o.list = o.list + o.other
PRINT o.list; o.other
SYSTEM
