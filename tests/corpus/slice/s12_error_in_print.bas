$CONSOLE:ONLY
' Errors raised inside a PRINT (spec language/error-handling, "Resume"; spec compiler/pipeline, "Error inside a
' PRINT"): RESUME re-runs the whole PRINT; RESUME NEXT skips the rest of it, line end included; an ERROR inside a
' FUNCTION called from a PRINT resumes inside the FUNCTION; an untrapped error skips the rest of its PRINT (run
' with QB64PE_NOPROMPT=continue, s12_error_in_print.noprompt, so the program goes on).
DIM k AS LONG
k = -1
ON ERROR GOTO fixk
PRINT "a"; CHR$(k); "b"
ON ERROR GOTO skip
PRINT "c"; CHR$(300); "d"
PRINT "e"; f&; "g"
PRINT "h"; fstr$("i"); CHR$(-1); "j"
PRINT CHR$(72); CHR$(255 - 150); CHR$(0 + 33)
ON ERROR GOTO 0
PRINT "untrapped"; CHR$(-5); "not printed"
PRINT "after untrapped"
SYSTEM

fixk:
PRINT "fixk"; ERR
k = 65
RESUME

skip:
PRINT "skip"; ERR
RESUME NEXT

FUNCTION f&
    f& = 1
    ERROR 7
    f& = 2
END FUNCTION

FUNCTION fstr$ (t AS STRING)
    fstr$ = t + t
END FUNCTION
