' TEST: cpp
$CONSOLE:ONLY
' ON n GOTO / ON n GOSUB as emitted (m2-core-builtins task 7.1): qbr_float_to_long of a float value, the label
' tests, the GOSUB return points, error(5) for a negative value
x = 2
ON x GOTO a, b
ON x GOSUB a, b
END
a: RETURN
b: RETURN
