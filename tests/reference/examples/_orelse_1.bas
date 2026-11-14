$CONSOLE:ONLY
PRINT "0 _ORELSE 5 is"; 0 _ORELSE 5
PRINT "First check:"; 1 _ORELSE Check(1)
PRINT "Second check:"; 0 _ORELSE Check(2)
END

FUNCTION Check (n)
    PRINT "(Check"; n; "runs)";
    Check = -1
END FUNCTION
