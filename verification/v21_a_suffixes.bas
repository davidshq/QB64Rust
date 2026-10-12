$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.1): the type suffixes of the new numeric types on implicit
' variables, on DIMmed names, and whether names differing only in the suffix are different variables.
ON ERROR GOTO h
a%% = -1: b~%% = -1: c~% = -1: d~& = -1: e~&& = -1: f%& = -1: g~%& = -1
PRINT "implicit -1:"; a%%; b~%%; c~%; d~&; e~&&; f%&; g~%&
h` = -1: i`5 = -1: j~` = -1: k~`5 = -1
PRINT "implicit bit -1:"; h`; i`5; j~`; k~`5
' Each `n above 32 is printed right after its store: such scalars overlap (D-009, v04_accidental).
m`40 = -1: PRINT "implicit `40 -1:"; m`40
n~`40 = -1: PRINT "implicit ~`40 -1:"; n~`40
a%% = 200: b~%% = 300: c~% = 70000: d~& = 4294967297: f%& = 9223372036854775807: g~%& = -2
PRINT "implicit wrap:"; a%%; b~%%; c~%; d~&; f%&; g~%&
i`5 = 17: k~`5 = 33
PRINT "implicit bit wrap:"; i`5; k~`5
m`40 = 549755813888: PRINT "implicit `40 549755813888:"; m`40
n~`40 = 1099511627776: PRINT "implicit ~`40 1099511627776:"; n~`40
PRINT "LEN implicit:"; LEN(a%%); LEN(b~%%); LEN(c~%); LEN(d~&); LEN(e~&&); LEN(f%&); LEN(g~%&)
DIM p%%, q~%%, r~%, s~&, t~&&, u%&, v~%&, w`, x`3, y~`, z~`3
p%% = 1: q~%% = 2: r~% = 3: s~& = 4: t~&& = 5: u%& = 6: v~%& = 7: w` = 1: x`3 = 3: y~` = 1: z~`3 = 7
PRINT "DIM suffix:"; p%%; q~%%; r~%; s~&; t~&&; u%&; v~%&; w`; x`3; y~`; z~`3
nm%% = 1: nm~%% = 2: nm% = 3: nm~% = 4: nm& = 5: nm~& = 6: nm&& = 7: nm~&& = 8: nm%& = 9: nm~%& = 10
nm` = -1: nm`2 = 1: nm~` = 1: nm~`2 = 3
PRINT "one name, each suffix:"; nm%%; nm~%%; nm%; nm~%; nm&; nm~&; nm&&; nm~&&; nm%&; nm~%&; nm`; nm`2; nm~`; nm~`2
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
