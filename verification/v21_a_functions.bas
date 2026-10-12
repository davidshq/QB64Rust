$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.1): FUNCTION names with the suffixes of the new numeric types:
' the result stored into the function's type, and the believed type of the call (HEX$ width).
ON ERROR GOTO h
PRINT "%%:"; fsb%%(200); fsb%%(-1)
PRINT "~%%:"; fub~%%(300); fub~%%(-1)
PRINT "~%:"; fui~%(70000); fui~%(-1)
PRINT "~&:"; ful~&(4294967297); ful~&(-1)
PRINT "~&&:"; fuq~&&(-1); fuq~&&(2.5)
PRINT "%&:"; fo%&(-1); fo%&(2.5)
PRINT "~%&:"; fuo~%&(-1)
PRINT "`3:"; fb3`3(5); fb3`3(3.5)
PRINT "~`3:"; fub3~`3(13); fub3~`3(-1)
PRINT "HEX$:"; HEX$(fsb%%(-2)); " "; HEX$(fub~%%(-2)); " "; HEX$(fui~%(-2)); " "; HEX$(fb3`3(-2))
PRINT "+ 1:"; fub~%%(255) + 1; fuq~&&(-1) + 1
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

FUNCTION fsb%% (x)
fsb%% = x
END FUNCTION

FUNCTION fub~%% (x AS DOUBLE)
fub~%% = x
END FUNCTION

FUNCTION fui~% (x AS DOUBLE)
fui~% = x
END FUNCTION

FUNCTION ful~& (x AS DOUBLE)
ful~& = x
END FUNCTION

FUNCTION fuq~&& (x AS DOUBLE)
fuq~&& = x
END FUNCTION

FUNCTION fo%& (x AS DOUBLE)
fo%& = x
END FUNCTION

FUNCTION fuo~%& (x AS DOUBLE)
fuo~%& = x
END FUNCTION

FUNCTION fb3`3 (x)
fb3`3 = x
END FUNCTION

FUNCTION fub3~`3 (x)
fub3~`3 = x
END FUNCTION
