$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): the library with three files in a program with a SUB of its own: the after-main file comes before the first SUB.
$USELIBRARY:'test/three'
PRINT "main "; top$
GOSUB three_handler
ThreeSub
Mine
SYSTEM

SUB Mine
    PRINT "mine"
END SUB
