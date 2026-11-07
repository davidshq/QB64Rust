$CONSOLE:ONLY
' Verification (OpenSpec change eval-order-left-to-right, task 2.1): the order in which PUT and GET evaluate their
' file number, record number and the index of their variable. The file is v69_i.tmp and is removed.
DIM SHARED x(1 TO 3) AS LONG
x(1) = 11: x(2) = 22: x(3) = 33
OPEN "v69_i.tmp" FOR RANDOM AS #1 LEN = 4
PRINT "PUT:";: PUT #f(1), f(2), x(f(3)): PRINT
PRINT "GET:";: GET #f(1), f(2), x(f(1)): PRINT
PRINT x(1); x(2); x(3)
CLOSE #1
KILL "v69_i.tmp"
SYSTEM

FUNCTION f& (k AS LONG)
    PRINT " f"; LTRIM$(STR$(k));
    f& = k
END FUNCTION
