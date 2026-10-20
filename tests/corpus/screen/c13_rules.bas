' The steps rule and play until of the keys file (tools/screen/README.md). Before each wait for a key the program
' writes the place of the wait to byte &H4F1 and a letter to &H4F2, then announces the wait in &H4F0. Month k
' brings k battles, each a wait at place 7; the second battle of a month asks at a menu (place 14, letter R) and
' then for a confirmation (place 5); the third month also passes a menu with the letter U; each month ends at the
' main menu (place 14, letter M). The keys file does not count the waits: it answers them by their places.
DIM k AS STRING, month AS INTEGER, b AS INTEGER
SCREEN 12
FOR month = 1 TO 3
    FOR b = 1 TO month
        PRINT "month"; month; "battle"; b;
        k = waitkey$(7, 0): PRINT " got "; shown$(k);
        IF b = 2 THEN
            k = waitkey$(14, ASC("R")): PRINT ", retreat menu got "; shown$(k);
            k = waitkey$(5, 0): PRINT ", confirm got "; shown$(k);
        END IF
        PRINT
    NEXT b
    IF month = 3 THEN k = waitkey$(14, ASC("U")): PRINT "utility menu got "; shown$(k)
    k = waitkey$(14, ASC("M")): PRINT "main menu got "; shown$(k)
NEXT month
SYSTEM

FUNCTION waitkey$ (place AS INTEGER, letter AS INTEGER)
    DIM k AS STRING
    DEF SEG = 0: POKE 1265, place: POKE 1266, letter: POKE 1264, (PEEK(1264) + 1) AND 255: DEF SEG
    DO: k = INKEY$: LOOP WHILE k = ""
    waitkey$ = k
END FUNCTION

FUNCTION shown$ (k AS STRING)
    IF ASC(k) < 33 THEN shown$ = "CHR$(" + LTRIM$(STR$(ASC(k))) + ")" ELSE shown$ = k
END FUNCTION
