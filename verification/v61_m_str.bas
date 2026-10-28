$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named STR.
TYPE t
    str AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.str = 4: o.other = 2
o.str = o.str + o.other
PRINT o.str; o.other
SYSTEM
