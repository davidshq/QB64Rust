$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named SUB.
TYPE t
    sub AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.sub = 4: o.other = 2
o.sub = o.sub + o.other
PRINT o.sub; o.other
SYSTEM
