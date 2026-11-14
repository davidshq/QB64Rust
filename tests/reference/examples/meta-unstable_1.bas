$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
TYPE Bag
    items(1 TO 2) _DYNAMIC AS LONG
END TYPE
DIM b AS Bag
PRINT "items runs from"; LBOUND(b.items); "to"; UBOUND(b.items)
REDIM b.items(1 TO 5) AS LONG
b.items(5) = 42
PRINT "After REDIM:"; UBOUND(b.items); b.items(5)
END
