$CONSOLE:ONLY
' Verification (m2-numeric-types, task 8.3): a constant used with another suffix than its own: a plain constant
' with the suffixes of the new numeric types, and a constant of a new type with an old or another new suffix.
' Each value printed and with + 0 (the held value), and compared with 0.
ON ERROR GOTO h
CONST big = 4294967295, neg = -1, fl = 2.5, small = 300
PRINT "plain as new:"; big~&; neg~&; neg~%%; small%%; small~%%; fl~%; neg~&&; neg%&; neg~%&
PRINT "+ 0:"; big~& + 0; neg~& + 0; neg~%% + 0; small%% + 0; small~%% + 0; fl~% + 0; neg~&& + 0
PRINT "< 0:"; neg~& < 0; neg~%% < 0; neg~&& < 0
CONST b~%% = -1, u~& = 4294967295, q%% = 200
PRINT "new as old:"; b~%%; b%; b&; b&&; b!; b#; u&; u%; q%; q&
PRINT "+ 0:"; b% + 0; b& + 0; b&& + 0; u& + 0; u% + 0; q% + 0
PRINT "new as new:"; b~&; b%%; u~%%; u~&&; q~%%; q~%
PRINT "+ 0:"; b~& + 0; b%% + 0; u~%% + 0; q~%% + 0
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
