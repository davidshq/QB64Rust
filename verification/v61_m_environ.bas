$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named ENVIRON.
TYPE t
    environ AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.environ = 4: o.other = 2
o.environ = o.environ + o.other
PRINT o.environ; o.other
SYSTEM
