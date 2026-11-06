$CONSOLE:ONLY
' v68 "Unused variable" for parameters never used: with a suffix, after a DECLARE SUB, with an AS clause, and in a
' SUB whose body holds only a comment (Civil War Strategy's `SUB TICK (sec!)` gets no warning).
DECLARE SUB Declared (p2)
DECLARE SUB Both (p5!)
Suffixed 1
Declared 2
Typed 3
Comments 4
Both 5
SYSTEM

SUB Suffixed (p1!)
    PRINT "suffixed"
END SUB

SUB Declared (p2)
    PRINT "declared"
END SUB

SUB Typed (p3 AS LONG)
    PRINT "typed"
END SUB

SUB Comments (p4)
    ' only a comment
END SUB

SUB Both (p5!)
    ' only a comment
END SUB
