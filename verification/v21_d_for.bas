$CONSOLE:ONLY
' Verification (m2-numeric-types, D1, task 1.4): FOR with each new numeric type as the variable: limits rounded
' to the hidden wider type, passing the end of the type's range (each loop capped at 6 passes), negative steps
' through 0 for unsigned types. The hidden types are read from the C++ (qb64pe -z).
ON ERROR GOTO h
DIM n AS INTEGER
n = 0: PRINT "~%% 250 TO 255 STEP 3:";
FOR b~%% = 250 TO 255 STEP 3: PRINT b~%%;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; b~%%
n = 0: PRINT "%% 120 TO 127 STEP 4:";
FOR s%% = 120 TO 127 STEP 4: PRINT s%%;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; s%%
n = 0: PRINT "~% 2 TO 0 STEP -1:";
FOR u~% = 2 TO 0 STEP -1: PRINT u~%;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; u~%
n = 0: PRINT "~% 65530 TO 65535 STEP 4:";
FOR u~% = 65530 TO 65535 STEP 4: PRINT u~%;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; u~%
n = 0: PRINT "~& 2 TO 0 STEP -1:";
FOR ul~& = 2 TO 0 STEP -1: PRINT ul~&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; ul~&
n = 0: PRINT "~& 4294967290 TO 4294967295 STEP 4:";
FOR ul~& = 4294967290 TO 4294967295 STEP 4: PRINT ul~&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; ul~&
n = 0: PRINT "~&& 2 TO 0 STEP -1:";
FOR uq~&& = 2 TO 0 STEP -1: PRINT uq~&&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; uq~&&
n = 0: PRINT "~&& 1 TO 18446744073709551615~&& STEP 9223372036854775807:";
FOR uq~&& = 1 TO 18446744073709551615~&& STEP 9223372036854775807: PRINT uq~&&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; uq~&&
n = 0: PRINT "%& 1 TO 3:";
FOR o%& = 1 TO 3: PRINT o%&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; o%&
n = 0: PRINT "~%& 2 TO 0 STEP -1:";
FOR uo~%& = 2 TO 0 STEP -1: PRINT uo~%&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; uo~%&
n = 0: PRINT "~%% 1 TO 2.5:";
FOR b~%% = 1 TO 2.5: PRINT b~%%;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; b~%%
n = 0: PRINT "~%% 0.5 TO 3 STEP 1.5:";
FOR b~%% = 0.5 TO 3 STEP 1.5: PRINT b~%%;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; b~%%
n = 0: PRINT "~% 1 TO 2.5000001#:";
FOR u~% = 1 TO 2.5000001#: PRINT u~%;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; u~%
n = 0: PRINT "~& 1 TO 2.5#:";
FOR ul~& = 1 TO 2.5#: PRINT ul~&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; ul~&
n = 0: PRINT "~& -1 TO 1:";
FOR ul~& = -1 TO 1: PRINT ul~&;: n = n + 1: IF n = 6 THEN EXIT FOR
NEXT: PRINT " end"; ul~&
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
