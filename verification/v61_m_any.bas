$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ANY.
TYPE t
    any AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.any = 4: o.other = 2
o.any = o.any + o.other
PRINT o.any; o.other
SYSTEM
