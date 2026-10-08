$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.4): SELECT CASE with each new numeric type as the selector: a plain
' variable (read at each test) and an expression (the hidden copy's type, read from the C++), with items that
' only match after conversion to the selector's type (-1 for an unsigned selector, 2.5 for an integer one).
ON ERROR GOTO h
DIM ub AS _UNSIGNED _BYTE, sb AS _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64
DIM so AS _OFFSET, uo AS _UNSIGNED _OFFSET, b3 AS _BIT * 3, u3 AS _UNSIGNED _BIT * 3
ub = 255: PRINT "~%% 255:";
SELECT CASE ub
    CASE -1: PRINT " -1"
    CASE 255: PRINT " 255"
    CASE ELSE: PRINT " else"
END SELECT
sb = -1: PRINT "%% -1:";
SELECT CASE sb
    CASE 255: PRINT " 255"
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
ui = 65535: PRINT "~% 65535:";
SELECT CASE ui
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
ui = 2: PRINT "~% 2:";
SELECT CASE ui
    CASE 2.5: PRINT " 2.5"
    CASE ELSE: PRINT " else"
END SELECT
ul = 4294967295: PRINT "~& max:";
SELECT CASE ul
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
ul = 4294967295: PRINT "~& max, IS > 0:";
SELECT CASE ul
    CASE IS < 0: PRINT " < 0"
    CASE IS > 0: PRINT " > 0"
END SELECT
uq = 18446744073709551615~&&: PRINT "~&& max:";
SELECT CASE uq
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
uq = 18446744073709551615~&&: PRINT "~&& max, 0 TO -1:";
SELECT CASE uq
    CASE 0 TO -1: PRINT " 0 TO -1"
    CASE ELSE: PRINT " else"
END SELECT
so = -1: PRINT "%& -1:";
SELECT CASE so
    CASE 18446744073709551615~&&: PRINT " max ~&&"
    CASE -1: PRINT " -1"
END SELECT
uo = 5: PRINT "~%& 5:";
SELECT CASE uo
    CASE 4.5: PRINT " 4.5"
    CASE ELSE: PRINT " else"
END SELECT
b3 = -3: PRINT "_BIT * 3 -3:";
SELECT CASE b3
    CASE 5: PRINT " 5"
    CASE -3: PRINT " -3"
END SELECT
u3 = 7: PRINT "_UNSIGNED _BIT * 3 7:";
SELECT CASE u3
    CASE -1: PRINT " -1"
    CASE 7: PRINT " 7"
END SELECT
' Expression selectors (hidden copy)
ub = 255: PRINT "~%% + 0 255:";
SELECT CASE ub + 0
    CASE -1: PRINT " -1"
    CASE 255: PRINT " 255"
END SELECT
ul = 4294967295: PRINT "~& + 0 max:";
SELECT CASE ul + 0
    CASE -1: PRINT " -1"
    CASE 4294967295: PRINT " 4294967295"
END SELECT
uq = 18446744073709551615~&&: PRINT "~&& + 0 max:";
SELECT CASE uq + 0
    CASE -1: PRINT " -1"
    CASE ELSE: PRINT " else"
END SELECT
uq = 18446744073709551615~&&: PRINT "~&& + 1~&& max:";
SELECT CASE uq + 1~&&
    CASE 0: PRINT " 0"
    CASE ELSE: PRINT " else"
END SELECT
b3 = -3: PRINT "_BIT * 3 + 0 -3:";
SELECT CASE b3 + 0
    CASE 5: PRINT " 5"
    CASE -3: PRINT " -3"
END SELECT
DIM a(2) AS _UNSIGNED INTEGER
a(1) = 65535: PRINT "~% element 65535:";
SELECT CASE a(1)
    CASE -1: PRINT " -1"
    CASE 65535: PRINT " 65535"
END SELECT
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
