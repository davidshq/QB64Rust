$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named PLAY.
TYPE t
    play AS INTEGER
    other AS INTEGER
END TYPE
DIM o AS t
o.play = 4: o.other = 2
o.play = o.play + o.other
PRINT o.play; o.other
SYSTEM
