$CONSOLE:ONLY
' Verification (m3-network-embed, by its review): a string for a number parameter of a user FUNCTION
x = f&("a")
SYSTEM
FUNCTION f& (a AS LONG)
    f& = a
END FUNCTION
