$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RETURN.
TYPE t
    return AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.return = 4: o.other = 2
o.return = o.return + o.other
PRINT o.return; o.other
SYSTEM
