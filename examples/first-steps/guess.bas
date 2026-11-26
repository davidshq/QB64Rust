$CONSOLE:ONLY
' Guess the number: the program thinks of a number from 1 to 100, and you guess it.
RANDOMIZE TIMER
secret = INT(RND * 100) + 1
tries = 0
PRINT "I am thinking of a number from 1 to 100."
DO
    INPUT "Your guess"; guess
    tries = tries + 1
    IF guess < secret THEN
        PRINT "Higher!"
    ELSEIF guess > secret THEN
        PRINT "Lower!"
    END IF
LOOP UNTIL guess = secret
PRINT "Right! You found it in"; tries; "tries."
END
