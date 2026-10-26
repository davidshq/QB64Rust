$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ABSOLUTE.
TYPE t
    absolute AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.absolute = 4: o.other = 2
o.absolute = o.absolute + o.other
PRINT o.absolute; o.other
SYSTEM
