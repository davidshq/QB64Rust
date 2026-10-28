$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named CALL.
TYPE t
    call AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.call = 4: o.other = 2
o.call = o.call + o.other
PRINT o.call; o.other
SYSTEM
