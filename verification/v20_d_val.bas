$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): VAL of edge texts, and VAL with a type.
ON ERROR GOTO h
PRINT "VAL 12:"; VAL("12"); " -3.5:"; VAL("-3.5"); " empty:"; VAL("")
PRINT "VAL &H:"; VAL("&H"); " &HFF:"; VAL("&HFF"); " &HFFFF:"; VAL("&HFFFF"); " &HFFFFFFFF:"; VAL("&HFFFFFFFF")
PRINT "VAL &O17:"; VAL("&O17"); " &O:"; VAL("&O"); " &B101:"; VAL("&B101"); " &B:"; VAL("&B")
PRINT "VAL blanks 1 2:"; VAL(" 1 2"); " tab 3:"; VAL(CHR$(9) + "3"); " 1e3:"; VAL("1e3"); " 1d3:"; VAL("1d3")
PRINT "VAL junk:"; VAL("abc"); " 12abc:"; VAL("12abc"); " +5:"; VAL("+5"); " .5:"; VAL(".5"); " 1,5:"; VAL("1,5")
PRINT "VAL 1e400:"; VAL("1e400"); " 1e4000:"; VAL("1e4000"); " -1e400:"; VAL("-1e400")
PRINT "VAL 0.1:"; VAL("0.1"); " 1/3 text:"; VAL("0.333333333333333333333333")
PRINT "VAL INTEGER 40000:"; VAL("40000", INTEGER); " 2.7:"; VAL("2.7", INTEGER); " -2.5:"; VAL("-2.5", INTEGER)
PRINT "VAL LONG 3000000000:"; VAL("3000000000", LONG); " 1e3:"; VAL("1e3", LONG); " &HFFFFFFFF:"; VAL("&HFFFFFFFF", LONG)
PRINT "VAL _INTEGER64 9007199254740993:"; VAL("9007199254740993", _INTEGER64); " 1.5:"; VAL("1.5", _INTEGER64)
PRINT "VAL SINGLE 0.1:"; VAL("0.1", SINGLE); " DOUBLE 0.1:"; VAL("0.1", DOUBLE); " _FLOAT 0.1:"; VAL("0.1", _FLOAT)
PRINT "VAL SINGLE 1e40:"; VAL("1e40", SINGLE); " DOUBLE 1e400:"; VAL("1e400", DOUBLE)
PRINT "VAL _INTEGER64 1e30:"; VAL("1e30", _INTEGER64); " junk:"; VAL("x", LONG)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
