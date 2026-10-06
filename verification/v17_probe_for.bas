$CONSOLE:ONLY
' Verification (m2-control-flow-slice, design Context, probe 2): FOR loops at type limits, with float steps,
' a body that changes the variable, a loop that does not run, a limit changed in the body, a step of 0.
FOR i% = 32760 TO 32767 STEP 4: PRINT i%;: NEXT: PRINT "after"; i%
FOR b& = 2147483640 TO 2147483647 STEP 5: PRINT b&;: NEXT: PRINT "after"; b&
n = 0: FOR q&& = 9223372036854775800 TO 9223372036854775807 STEP 5: PRINT q&&;: n = n + 1: IF n > 4 THEN EXIT FOR
NEXT: PRINT "after (capped at 5 passes)"; q&&
FOR u%% = 120 TO 127 STEP 5: PRINT u%%;: NEXT: PRINT "after"; u%%
c = 0: FOR s! = 0 TO 1 STEP 0.1: c = c + 1: NEXT: PRINT "single steps:"; s!; c
c = 0: FOR d# = 0 TO 1 STEP 0.1: c = c + 1: NEXT: PRINT "double steps:"; d#; c
FOR k = 1 TO 5: k = k + 1: PRINT k;: NEXT: PRINT
FOR m = 3 TO 1: PRINT "never": NEXT: PRINT "not run:"; m
e = 3: FOR m = 1 TO e: e = 10: PRINT m;: NEXT: PRINT "after"; m
n = 0: FOR z = 1 TO 2 STEP 0: z = z + 1: PRINT z;: n = n + 1: IF n > 5 THEN EXIT FOR
NEXT: PRINT "after"; z
FOR m = 5 TO 1 STEP -2: PRINT m;: NEXT: PRINT "after"; m
FOR m = 1.5 TO 3: PRINT m;: NEXT: PRINT "after"; m
FOR i% = 1 TO 2.6: PRINT i%;: NEXT: PRINT "after"; i%
FOR i% = 1 TO 3 STEP 0.6: PRINT i%;: NEXT: PRINT "after"; i%
FOR i% = 3 TO 1 STEP -0.6: PRINT i%;: NEXT: PRINT "after"; i%
SYSTEM
