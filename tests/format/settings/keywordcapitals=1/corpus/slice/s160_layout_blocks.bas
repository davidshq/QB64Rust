$CONSOLE:ONLY
' Slice (m5b-format-settings, task 1.1): blocks of every kind laid out by hand, for the format oracle under the
' other layout settings (tests\format\settings.list). The indentation is written wrong on purpose, the spacing
' is odd, the keywords are in mixed case, and each block line has a comment after it.
TYPE pair ' a record
    a AS INTEGER ' first
    b AS STRING * 3
END TYPE ' end of the record
DIM p AS pair: p.a = 7: p.b = "xyz"
PRINT p.a; p.b
i% = 0
DO WHILE i% < 3 ' a loop tested first
    i% = i% + 1
    IF i% = 2 THEN ' a block IF
        PRINT "two"
    ELSEIF i% = 3 THEN ' its ELSEIF
        PRINT "three"
    ELSE ' its ELSE
        PRINT "one"
    END IF ' its END IF
LOOP ' end of the loop
DO ' a loop tested last
    i% = i% - 1
LOOP UNTIL i% <= 0 ' its end
FOR j = 1 TO 3 STEP 1 ' a FOR
    FOR k = 1 TO j ' a nested FOR
        PRINT j * k;
    NEXT k ' its NEXT
    PRINT
NEXT ' end of the outer FOR
WHILE j > 1 ' a WHILE
    j = j - 1
WEND ' its WEND
FOR n = 1 TO 5
    SELECT CASE n ' a SELECT CASE
        CASE 1 ' a CASE
            PRINT "a";
        CASE 2, 3 ' a CASE with two values
            PRINT "b";
        CASE IS > 4 ' CASE IS
            PRINT "c";
        CASE ELSE ' CASE ELSE
            PRINT "d";
    END SELECT ' end of the SELECT
NEXT n
PRINT
IF j = 1 THEN PRINT "single line" ELSE PRINT "not" ' a single-line IF
IF j = 1 THEN
    PRINT "joined";: PRINT " in a block": i% = 0
END IF
FOR q = 1 TO 2: PRINT q;: NEXT: PRINT
10 PRINT "line ten" ' a line number
20 IF j = 1 THEN GOTO done ' a jump
PRINT "not reached"
done: ' a label
PRINT "done"
again: PRINT "label and statement"
