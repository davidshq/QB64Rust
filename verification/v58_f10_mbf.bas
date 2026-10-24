$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): MKSMBF$ and MKDMBF$ (their bytes in hex) of 0, 1.5, -2.25, a large and a tiny
' value and an integer, CVSMBF and CVDMBF back, their lengths, a value too large for the format and a string too
' short under a handler, a string longer than the format.
ON ERROR GOTO handler
PRINT hx$(MKSMBF$(0)); hx$(MKSMBF$(1.5)); hx$(MKSMBF$(-2.25)); hx$(MKSMBF$(1E30)); hx$(MKSMBF$(1E-30))
PRINT hx$(MKDMBF$(0)); hx$(MKDMBF$(1.5#)); hx$(MKDMBF$(-2.25#)); hx$(MKDMBF$(1D30)); hx$(MKDMBF$(1D-30))
PRINT CVSMBF(MKSMBF$(1.5)); CVDMBF(MKDMBF$(-2.25#)); CVSMBF(MKSMBF$(1E30)); CVDMBF(MKDMBF$(1D-30))
PRINT LEN(MKSMBF$(1)); LEN(MKDMBF$(1))
d# = 3.75: PRINT hx$(MKSMBF$(d#)); CVSMBF(MKSMBF$(d#))
i% = 3: PRINT hx$(MKSMBF$(i%)); hx$(MKDMBF$(i%))
f## = 1.25: PRINT hx$(MKDMBF$(f##)); CVDMBF(MKDMBF$(f##))
PRINT CVSMBF(MKSMBF$(1.5) + "zz"); CVDMBF(MKDMBF$(1.5#) + "zz")
s$ = MKSMBF$(3E38): PRINT LEN(s$)
s$ = MKDMBF$(1D300): PRINT LEN(s$)
x! = 7: x! = CVSMBF("ab"): PRINT x!
x# = 7: x# = CVDMBF("abcd"): PRINT x#
x# = CVDMBF(MKSMBF$(1.5) + MKSMBF$(1.5)): PRINT x#
y&& = CVSMBF(MKSMBF$(2.5)): PRINT y&&
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT

FUNCTION hx$ (s$)
    t$ = ""
    FOR i = 1 TO LEN(s$)
        t$ = t$ + RIGHT$("0" + HEX$(ASC(s$, i)), 2)
    NEXT
    hx$ = t$ + "|"
END FUNCTION
