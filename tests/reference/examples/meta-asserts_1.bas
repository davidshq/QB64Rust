$CONSOLE:ONLY
$ASSERTS
x = 5
_ASSERT x > 0, "x must be positive"
PRINT "The assertion held."
$IF _ASSERTS_ THEN
    PRINT "Assertions are on."
$END IF
END
