$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named NEXT.
TYPE t
    next AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.next = 4: o.other = 2
o.next = o.next + o.other
PRINT o.next; o.other
SYSTEM
