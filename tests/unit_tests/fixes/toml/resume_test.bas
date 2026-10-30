' Spec compiler/unit-tests, "The fixes of the test file's folder": qb64rust.toml beside this file turns
' resume-after-block on, so RESUME NEXT after the error in the WHILE's condition goes on after the WEND.
ON ERROR GOTO handler
GOTO ready
handler:
RESUME NEXT
ready:

SUB TestWhile
    DIM k AS LONG, s AS STRING
    k = -1
    WHILE LEN(CHR$(k)) = 1: s = s + "body": k = 65: WEND
    s = s + "after"
    AssertEqualStr "after", s, "resume-after-block"
END SUB
