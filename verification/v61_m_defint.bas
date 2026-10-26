$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DEFINT.
TYPE t
    defint AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.defint = 4: o.other = 2
o.defint = o.defint + o.other
PRINT o.defint; o.other
SYSTEM
