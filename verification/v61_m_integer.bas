$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named INTEGER.
TYPE t
    integer AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.integer = 4: o.other = 2
o.integer = o.integer + o.other
PRINT o.integer; o.other
SYSTEM
