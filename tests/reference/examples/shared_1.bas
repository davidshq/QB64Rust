$CONSOLE:ONLY
DIM SHARED turn AS INTEGER
DIM gold AS LONG
gold = 500
NextTurn
NextTurn
PRINT "After two turns: turn ="; turn; "and gold ="; gold
END

SUB NextTurn
    SHARED gold AS LONG
    turn = turn + 1
    gold = gold - 100
END SUB
