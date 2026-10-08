$CONSOLE:ONLY
' Verification (m2-numeric-types, task 1.6): the scenarios added to the spec deltas by task 1.6, each exactly as
' written there (numeric-semantics, constants, procedures), so the expected output in the deltas is measured.
PRINT 300~%%; -1~&; 40000%; 300~%% + 0; -1~& + 0
l& = 300~%%: PRINT l&
PRINT 4294967295~& + 1; -1~&& + 0; -1~&& > 0; 255~%% = -1%%
PRINT 1`; 9`3; 9`3 + 0; HEX$(-1`5)
PRINT &H1FF~%%; &H1FF~%% + 0
CONST u~% = -1, c~%% = 300: PRINT u~%; u~% + 0; u~% < 0; c~%%; c~%% + 0
CONST i`3 = 5, j~`3 = -1: PRINT i`3; j~`3; i`3 + 0
s$ = "hello world"
sp s$
PRINT s$
PRINT "["; fs$5("ab"); "]"; LEN(fs$5("ab"))
SYSTEM

SUB sp (t AS STRING * 5)
    PRINT "["; t; "]"; LEN(t); LEN(t + "!")
    t = "much longer text"
END SUB

FUNCTION fs$5 (x AS STRING)
    fs$5 = x
END FUNCTION
