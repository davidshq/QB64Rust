$CONSOLE:ONLY
' Verification (study 44, section 2 item 5): ON ERROR GOTO a label whose name starts with `__`, from a SUB, as in
' the Libraries Pack's a740g/Catch (Catch.bm line 262). A single leading `_` is "Invalid name" (v50_x28).
s
PRINT "after"
SYSTEM

__h:
PRINT "handled"; ERR
RESUME NEXT

SUB s
    ON ERROR GOTO __h
    ERROR 5
END SUB
