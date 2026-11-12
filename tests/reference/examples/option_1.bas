$CONSOLE:ONLY
OPTION _EXPLICIT
OPTION BASE 1
DIM days(7) AS STRING
DIM i AS INTEGER
PRINT "days runs from"; LBOUND(days); "to"; UBOUND(days)
DIM hours(0 TO 23) AS INTEGER
PRINT "hours runs from"; LBOUND(hours); "to"; UBOUND(hours)
FOR i = 1 TO 3
    days(i) = "day" + STR$(i)
NEXT i
PRINT days(1); days(3)
END
