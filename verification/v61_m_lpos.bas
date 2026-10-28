$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LPOS.
TYPE t
    lpos AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.lpos = 4: o.other = 2
o.lpos = o.lpos + o.other
PRINT o.lpos; o.other
SYSTEM
