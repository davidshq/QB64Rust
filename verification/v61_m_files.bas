$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named FILES.
TYPE t
    files AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.files = 4: o.other = 2
o.files = o.files + o.other
PRINT o.files; o.other
SYSTEM
