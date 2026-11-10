$CONSOLE:ONLY
' Verification (study 44, section 2 item 2): a number suffix on a `STRING * n` member.
TYPE t
    s AS STRING * 4
END TYPE
DIM v AS t
v.s% = 1
PRINT v.s
SYSTEM
