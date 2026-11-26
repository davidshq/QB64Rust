$CONSOLE:ONLY
' Higher or lower: a card from 1 to 13 is shown. Say whether the next one is higher or lower. A wrong
' answer ends the game.
RANDOMIZE TIMER
card = INT(RND * 13) + 1
score = 0
DO
    PRINT "The card is"; card
    INPUT "Higher or lower (H or L)"; answer$
    answer$ = UCASE$(LEFT$(answer$, 1))
    nextCard = INT(RND * 13) + 1
    PRINT "The next card is"; nextCard
    correct = (answer$ = "H" AND nextCard > card) OR (answer$ = "L" AND nextCard < card)
    IF correct THEN
        score = score + 1
        PRINT "Right!"
    ELSEIF nextCard = card THEN
        PRINT "The same card: no point."
        correct = -1
    ELSE
        PRINT "Wrong."
    END IF
    card = nextCard
LOOP WHILE correct
PRINT "Game over. You were right"; score; "times."
END
