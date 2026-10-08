' TEST: check-fail
$CONSOLE:ONLY
' ON n GOTO / ON n GOSUB rejected (verification\v20_x18_on_string: "Expected numeric expression";
' v20_x19_on_other_body: "Label 'l1' not defined"); a line number as a target is not supported yet
s$ = "a"
ON s$ GOTO here
ON 1 GOTO l1
ON 1 GOSUB here, 10
here:
SUB p
l1:
END SUB
