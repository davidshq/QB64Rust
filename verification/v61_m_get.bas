$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named GET.
TYPE t
    get AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.get = 4: o.other = 2
o.get = o.get + o.other
PRINT o.get; o.other
SYSTEM
