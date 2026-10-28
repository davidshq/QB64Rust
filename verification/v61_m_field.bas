$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named FIELD.
TYPE t
    field AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.field = 4: o.other = 2
o.field = o.field + o.other
PRINT o.field; o.other
SYSTEM
