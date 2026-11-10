$CONSOLE:ONLY
' Verification (study 44, section 2 item 4): a stray comma in a DECLARE FUNCTION's parameter list, as in the
' Libraries Pack's MattKilgore/QB64pe-Json/json.bi line 128 (`idx, AS LONG`).
DECLARE FUNCTION f& (a AS LONG, b, AS LONG)
PRINT f&(1, 2)
SYSTEM

FUNCTION f& (a AS LONG, b AS LONG)
    f& = a + b
END FUNCTION
