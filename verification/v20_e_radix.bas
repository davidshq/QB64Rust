$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): HEX$, OCT$, _BIN$ of each type, of negative values, of floats; the width of a
' negative value: a variable, an element, a member, an expression, a call.
ON ERROR GOTO h
TYPE t
    m AS INTEGER
END TYPE
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, f AS SINGLE, d AS DOUBLE, x AS _FLOAT
DIM a(2) AS INTEGER, u AS t
i = -2: l = -2: q = -2: a(1) = -2: u.m = -2
PRINT "HEX$ vars: "; HEX$(i); " "; HEX$(l); " "; HEX$(q); " element: "; HEX$(a(1)); " member: "; HEX$(u.m)
PRINT "OCT$ vars: "; OCT$(i); " "; OCT$(l); " "; OCT$(q); " element: "; OCT$(a(1)); " member: "; OCT$(u.m)
PRINT "_BIN$ vars: "; _BIN$(i); " "; _BIN$(l); " "; _BIN$(q)
PRINT "HEX$ exprs: "; HEX$(i * 1); " "; HEX$(l * 1); " "; HEX$(q * 1); " "; HEX$(i AND i); " "; HEX$(-i)
PRINT "OCT$ exprs: "; OCT$(i * 1); " "; OCT$(l * 1); " "; OCT$(q * 1)
PRINT "_BIN$ exprs: "; _BIN$(i * 1); " "; _BIN$(l * 1); " "; _BIN$(q * 1)
i = -1: l = -1: q = -1
PRINT "HEX$ -1 exprs: ["; HEX$(i * 1); "]["; HEX$(l * 1); "]["; HEX$(q * 1); "]["; HEX$(q); "]"
PRINT "OCT$ -1 exprs: ["; OCT$(i * 1); "]["; OCT$(l * 1); "]["; OCT$(q * 1); "]"
PRINT "_BIN$ -1 exprs: ["; _BIN$(i * 1); "]["; _BIN$(l * 1); "]["; _BIN$(q * 1); "]"
PRINT "HEX$ literals: "; HEX$(-1); " "; HEX$(-1%); " "; HEX$(-1&); " "; HEX$(-1&&); " "; HEX$(255); " "; HEX$(-70000)
PRINT "HEX$ calls: "; HEX$(CINT(-2)); " "; HEX$(CLNG(-2)); " "; HEX$(SGN(-2)); " "; HEX$(ASC("a") - 200)
PRINT "HEX$ 0: "; HEX$(0); " OCT$ 0: "; OCT$(0); " _BIN$ 0: "; _BIN$(0)
f = -2.5: d = 2.5: x = 1E+30
PRINT "HEX$ floats: "; HEX$(f); " "; HEX$(d); " "; HEX$(-1.5); " "; HEX$(3.5#)
PRINT "OCT$ floats: "; OCT$(f); " "; OCT$(d); " _BIN$ floats: "; _BIN$(f); " "; _BIN$(d)
PRINT "HEX$ 1E30: ["; HEX$(x); "]"
PRINT "OCT$ 1E30: ["; OCT$(x); "]"
PRINT "_BIN$ 1E30: ["; _BIN$(x); "]"
PRINT "HEX$ 2^62: "; HEX$(2## ^ 62); " -2^62: "; HEX$(-(2## ^ 62))
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT
