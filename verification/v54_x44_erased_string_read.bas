$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, review): does the old compiler accept this, and what does it do? (44_erased_string_read)
REDIM s(2) AS STRING
ERASE s
DATA "xyz"
READ s()
PRINT "after READ"
PRINT "end"
SYSTEM
