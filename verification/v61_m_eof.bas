$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named EOF.
TYPE t
    eof AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.eof = 4: o.other = 2
o.eof = o.eof + o.other
PRINT o.eof; o.other
SYSTEM
