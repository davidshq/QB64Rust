$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named VIEW.
TYPE t
    view AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.view = 4: o.other = 2
o.view = o.view + o.other
PRINT o.view; o.other
SYSTEM
