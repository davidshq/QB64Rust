$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CLS.
TYPE t
    cls AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.cls = 4: o.other = 2
o.cls = o.cls + o.other
PRINT o.cls; o.other
SYSTEM
