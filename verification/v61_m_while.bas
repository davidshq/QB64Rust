$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named WHILE.
TYPE t
    while AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.while = 4: o.other = 2
o.while = o.while + o.other
PRINT o.while; o.other
SYSTEM
