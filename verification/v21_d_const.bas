$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.4): CONST with the suffixes of the new numeric types: the value
' converted to the suffix's type, out of range, and how a use of the constant behaves in an expression
' (converted value or raw value) and in HEX$ (believed type).
ON ERROR GOTO h
CONST a%% = 200, b~%% = -1, c~%% = 300, d~% = -1, e~& = -1, f~&& = -1, g%& = 5, k~%& = -1
PRINT "values:"; a%%; b~%%; c~%%; d~%; e~&; f~&&; g%&; k~%&
PRINT "+ 0:"; a%% + 0; b~%% + 0; c~%% + 0; d~% + 0; e~& + 0; f~&& + 0; g%& + 0; k~%& + 0
PRINT "< 0:"; b~%% < 0; e~& < 0; f~&& < 0
PRINT "HEX$:"; HEX$(a%%); " "; HEX$(b~%%); " "; HEX$(d~%); " "; HEX$(e~&); " "; HEX$(f~&&)
CONST h` = 1, i`3 = 5, j~`3 = -1, m~` = 3
PRINT "bit:"; h`; i`3; j~`3; m~`
PRINT "bit + 0:"; h` + 0; i`3 + 0; j~`3 + 0; m~` + 0
CONST p = 5~%%, q = -1~&, r = 300~%%, s = 5`, t = &HFF%%
PRINT "suffixed literal values:"; p; q; r; s; t
PRINT "suffixed literal + 0:"; p + 0; q + 0; r + 0; s + 0; t + 0
CONST u%% = 2.5, v~% = 65535.5, w~&& = 1.5E+19
PRINT "float values:"; u%%; v~%; w~&&
CONST x~& = 4294967295, y = x~& + 1
PRINT "x~& + 1:"; y
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
