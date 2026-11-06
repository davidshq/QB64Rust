$CONSOLE:ONLY
' Verification (DIVERGENCES-QB45.md Q-018): the main module's GOSUB calls a SUB. The SUB makes its own
' GOSUB, leaves it by GOTO and ends at END SUB without RETURN. Then the main module's RETURN.
' One GOSUB stack for the whole program: that RETURN takes the SUB's entry and raises error 3.
' QB 4.5 and PDS 7.1 drop the SUB's entry and return to main (verification\qb45\q45_gosub_left_in_sub).
' The World War 2 game's editor stopped this way: its menu SUB leaves GOSUB selex by GOTO.
ON ERROR GOTO trap
PRINT "start"
GOSUB ask
PRINT "back in main"
SYSTEM
ask:
leaver
PRINT "after the CALL"
RETURN
trap:
PRINT "error"; ERR
SYSTEM

SUB leaver
    GOSUB inner
    PRINT "not reached"
    inner:
    GOTO done
    done:
    PRINT "leaving the SUB"
END SUB
