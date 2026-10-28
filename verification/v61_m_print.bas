$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PRINT.
TYPE t
    print AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.print = 4: o.other = 2
o.print = o.print + o.other
PRINT o.print; o.other
SYSTEM
