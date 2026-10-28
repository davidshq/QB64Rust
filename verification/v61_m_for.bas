$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named FOR.
TYPE t
    for AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.for = 4: o.other = 2
o.for = o.for + o.other
PRINT o.for; o.other
SYSTEM
