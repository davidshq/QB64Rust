$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named INTERRUPT.
TYPE t
    interrupt AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.interrupt = 4: o.other = 2
o.interrupt = o.interrupt + o.other
PRINT o.interrupt; o.other
SYSTEM
