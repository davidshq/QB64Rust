$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named DEFSTR.
TYPE t
    defstr AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.defstr = 4: o.other = 2
o.defstr = o.defstr + o.other
PRINT o.defstr; o.other
SYSTEM
