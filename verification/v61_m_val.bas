$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named VAL.
TYPE t
    val AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.val = 4: o.other = 2
o.val = o.val + o.other
PRINT o.val; o.other
SYSTEM
