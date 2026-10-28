$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named WEND.
TYPE t
    wend AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.wend = 4: o.other = 2
o.wend = o.wend + o.other
PRINT o.wend; o.other
SYSTEM
