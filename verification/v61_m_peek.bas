$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PEEK.
TYPE t
    peek AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.peek = 4: o.other = 2
o.peek = o.peek + o.other
PRINT o.peek; o.other
SYSTEM
