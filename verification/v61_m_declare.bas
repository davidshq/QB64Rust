$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DECLARE.
TYPE t
    declare AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.declare = 4: o.other = 2
o.declare = o.declare + o.other
PRINT o.declare; o.other
SYSTEM
