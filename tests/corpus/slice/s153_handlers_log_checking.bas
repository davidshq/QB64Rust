$CONSOLE:ONLY
' Slice (m4-handlers-logging-checking task 4.2): the handler chain (_NEWHANDLER in a SUB, _LASTHANDLER back), the
' four log statements and _LOGMINLEVEL (the runner logs to a file, so it is 2; the entries are not compared here,
' verification\v50_b_log compares them), and $CHECKING:OFF around an error, an index and a _MEMGET.
PRINT "min level"; _LOGMINLEVEL
_LOGTRACE "trace"
_LOGINFO "info"
_LOGWARN "warn"
_LOGERROR "error"
ON ERROR GOTO outer
ERROR 5
inner_test
ERROR 6
DIM a(1 TO 3) AS LONG, m AS _MEM, i AS LONG
m = _MEM(a())
$CHECKING:OFF
FOR i = 1 TO 3: a(i) = i * 7: NEXT
_MEMPUT m, m.OFFSET + 4, 99 AS LONG
ERROR 7
PRINT "not printed while the error is pending"
$CHECKING:ON
PRINT "the error is serviced here, and RESUME NEXT goes on after this line"
PRINT "checked"; a(1); a(2); a(3); _MEMGET(m, m.OFFSET + 8, LONG)
_MEMFREE m
SYSTEM

outer:
PRINT "outer"; ERR
RESUME NEXT
inner:
PRINT "inner"; ERR
RESUME NEXT

SUB inner_test
    ON ERROR GOTO _NEWHANDLER inner
    ERROR 9
    ON ERROR GOTO _LASTHANDLER
END SUB
