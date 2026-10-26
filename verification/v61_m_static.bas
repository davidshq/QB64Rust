$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named STATIC.
TYPE t
    static AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.static = 4: o.other = 2
o.static = o.static + o.other
PRINT o.static; o.other
SYSTEM
