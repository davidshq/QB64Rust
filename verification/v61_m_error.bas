$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ERROR.
TYPE t
    error AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.error = 4: o.other = 2
o.error = o.error + o.other
PRINT o.error; o.other
SYSTEM
