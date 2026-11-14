$CONSOLE:ONLY
PRINT "5 XOR 3 is"; 5 XOR 3
light = 0
FOR i = 1 TO 3
    light = light XOR 1
    PRINT "Switch"; i; ": the light is"; light
NEXT i
END
