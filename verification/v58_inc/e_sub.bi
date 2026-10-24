' Verification v58: a SUB that runs out of stack, in an included file.
SUB deepinc (n AS LONG)
    deepinc n + 1
END SUB
