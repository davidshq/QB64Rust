$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named RESUME.
TYPE t
    resume AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.resume = 4: o.other = 2
o.resume = o.resume + o.other
PRINT o.resume; o.other
SYSTEM
