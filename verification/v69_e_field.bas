$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which FIELD evaluates its file
' number, each width and the index of each target. The file is v69_e.tmp and is removed.
DIM SHARED e(1 TO 4) AS STRING
OPEN "v69_e.tmp" FOR RANDOM AS #1 LEN = 12
PRINT "FIELD #fn, wd AS e(ix):";
FIELD #fn, wd AS e(ix)
PRINT
PRINT LEN(e(1)); LEN(e(2))
PRINT "two parts:";
FIELD #fn, w(3) AS e(i(1)), w(4) AS e(i(2))
PRINT
PRINT LEN(e(1)); LEN(e(2))
CLOSE #1
KILL "v69_e.tmp"
SYSTEM

FUNCTION fn&
    PRINT " fn";
    fn& = 1
END FUNCTION

FUNCTION wd&
    PRINT " wd";
    wd& = 5
END FUNCTION

FUNCTION ix&
    PRINT " ix";
    ix& = 2
END FUNCTION

FUNCTION w& (k AS LONG)
    PRINT " w"; LTRIM$(STR$(k));
    w& = k
END FUNCTION

FUNCTION i& (k AS LONG)
    PRINT " i"; LTRIM$(STR$(k));
    i& = k
END FUNCTION
