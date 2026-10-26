$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named LOCATE.
TYPE t
    locate AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.locate = 4: o.other = 2
o.locate = o.locate + o.other
PRINT o.locate; o.other
SYSTEM
