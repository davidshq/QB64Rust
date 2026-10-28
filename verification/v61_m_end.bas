$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named END.
TYPE t
    end AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.end = 4: o.other = 2
o.end = o.end + o.other
PRINT o.end; o.other
SYSTEM
