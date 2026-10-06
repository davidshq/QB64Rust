$CONSOLE:ONLY
' FOR (spec language/control-flow, "FOR loops", "NEXT variables", "EXIT from loops"): the probe rows of the
' design (limits evaluated once, the body changing the variable, a loop that does not run, a step of 0), negative
' and float steps, start, limit and step rounded to the wider type, SINGLE, DOUBLE and _FLOAT variables, the
' integer types at their limits (INTEGER and LONG pass their range and end, _INTEGER64 wraps and is capped),
' NEXT j, i, NEXT with a suffix, EXIT FOR from inside an IF and from a nested FOR, FOR in a SUB (temporaries per
' call).
DIM ml AS LONG, q AS _INTEGER64
PRINT "plain"
FOR i = 1 TO 3: PRINT i;: NEXT: PRINT i
PRINT "negative step"
FOR i = 5 TO 1 STEP -2: PRINT i;: NEXT: PRINT i
PRINT "float start, plain SINGLE variable"
FOR i = 1.5 TO 3: PRINT i;: NEXT: PRINT i
PRINT "SINGLE steps"
c = 0: FOR s! = 0 TO 1 STEP 0.1: c = c + 1: NEXT: PRINT s!; c
PRINT "DOUBLE steps"
c = 0: FOR d# = 0 TO 1 STEP 0.1: c = c + 1: NEXT: PRINT d#; c
FOR d# = 1 TO 0 STEP -0.25: PRINT d#;: NEXT: PRINT d#
PRINT "_FLOAT steps"
FOR f## = 1 TO 2 STEP 0.5: PRINT f##;: NEXT: PRINT f##
PRINT "rounded to the wider type"
FOR i% = 1 TO 2.6: PRINT i%;: NEXT: PRINT i%
FOR i% = 0.5 TO 2: PRINT i%;: NEXT: PRINT i%
FOR i% = 1 TO 4 STEP 1.5: PRINT i%;: NEXT: PRINT i%
FOR ml = 0.5 TO 3.5: PRINT ml;: NEXT: PRINT ml
PRINT "limits evaluated once"
e = 3: FOR m = 1 TO e: e = 10: PRINT m;: NEXT: PRINT
st = 1: FOR m = 1 TO 6 STEP st: st = 3: PRINT m;: NEXT: PRINT
PRINT "the body changes the variable"
FOR k = 1 TO 5: k = k + 1: PRINT k;: NEXT: PRINT
PRINT "loops that do not run"
FOR m = 3 TO 1: PRINT "never": NEXT: PRINT m
FOR m = 1 TO 3 STEP -1: PRINT "never": NEXT: PRINT m
PRINT "step 0"
FOR z = 1 TO 2 STEP 0: z = z + 1: PRINT z;: NEXT: PRINT z
PRINT "INTEGER passes its range"
FOR i% = 32760 TO 32767 STEP 4: PRINT i%;: NEXT: PRINT i%
FOR i% = -32760 TO -32768 STEP -4: PRINT i%;: NEXT: PRINT i%
PRINT "LONG passes its range"
FOR ml = 2147483640 TO 2147483647 STEP 5: PRINT ml;: NEXT: PRINT ml
PRINT "_INTEGER64 wraps (capped at five passes)"
n = 0
FOR q = 9223372036854775800 TO 9223372036854775807 STEP 5
    PRINT q;
    n = n + 1
    IF n = 5 THEN EXIT FOR
NEXT
PRINT
PRINT q
PRINT "NEXT j, i"
FOR i = 1 TO 2
    FOR j = 1 TO 2
        PRINT i; j;
NEXT j, i
PRINT
PRINT i; j
PRINT "NEXT with a suffix"
FOR i = 1 TO 2: NEXT i!: PRINT i
FOR ml = 1 TO 2: NEXT ml&: PRINT ml
PRINT "EXIT FOR from inside an IF"
FOR i = 1 TO 9
    IF i = 3 THEN EXIT FOR
NEXT
PRINT i
PRINT "EXIT FOR leaves the inner FOR only"
FOR i = 1 TO 2
    FOR j = 1 TO 9
        IF j = 2 THEN EXIT FOR
    NEXT
    PRINT i; j;
NEXT
PRINT
PRINT "FOR in a SUB"
countdown 3
countdown 2
SYSTEM

SUB countdown (n AS INTEGER)
    FOR c% = n TO 1 STEP -1
        PRINT c%;
    NEXT
    PRINT "/"; c%
END SUB
