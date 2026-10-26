$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ERR.
TYPE t
    err AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.err = 4: o.other = 2
o.err = o.err + o.other
PRINT o.err; o.other
SYSTEM
