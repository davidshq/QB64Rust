' TEST: check-fail
$CONSOLE:ONLY
' Values of the new numeric types are "not supported yet" until m2-numeric-types task groups 5 and 6 give them their
' typing rules (the checker's gate, task group 4): a store, a load, a literal or a constant believed or held in one,
' an implicit variable, a FUNCTION result, an argument, a `FOR` variable. Their arrays and `TYPE` members come with
' task 8.1. The declarations themselves are accepted (`numeric_decls.bas`), so the follow-on rule does not start and
' the real error after the gated lines is reported (the array and the member are declarations marked "not supported
' yet": they start it, so they come last).
DIM b AS _UNSIGNED _BYTE
b = 5
PRINT b
PRINT 300~%%
PRINT -1~&&
PRINT 9`3
PRINT &HFF%%
CONST c~%% = 5, big = 4294967295
PRINT c~%%
PRINT big~&
x~% = 1
PRINT f%%
FOR u~& = 1 TO 2: NEXT
takes 5
x = "real error"
DIM a(3) AS _BYTE
TYPE t
    m AS _UNSIGNED LONG
END TYPE
SYSTEM

FUNCTION f%%
END FUNCTION

SUB takes (p AS _UNSIGNED INTEGER)
END SUB
