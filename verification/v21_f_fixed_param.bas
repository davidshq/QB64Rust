$CONSOLE:ONLY
' Verification (m2-numeric-types, task 1.6): a parameter declared STRING * n beyond LEN (v21_c_fixed_args: an
' ordinary STRING parameter whose LEN is n): built-ins on it, comparison, concatenation, passing it on, a store
' and LEN after it, a FUNCTION parameter, the suffix form, and SELECT CASE on it.
s$ = "hello world"
p s$
PRINT "after p s$: ["; s$; "]"
p "ab"
DIM f AS STRING * 3
f = "xyz"
p f
PRINT "after p f: ["; f; "]"
PRINT "g$: "; g$("abcdefgh")
w$ = "abc"
r w$
PRINT "after r w$: ["; w$; "]"
SYSTEM

SUB p (t AS STRING * 5)
    PRINT "["; t; "]"; LEN(t); LEN(t + "!"); LEN(RTRIM$(t)); "["; RIGHT$(t, 2); "]["; MID$(t, 2, 2); "]"; INSTR(t, "o")
    PRINT "compare:"; t = "hello"; t = "hello world"; t = "ab"; t = "ab   "
    SELECT CASE t
        CASE "ab   ": PRINT "case ab+3 blanks"
        CASE "ab": PRINT "case ab"
        CASE ELSE: PRINT "case else"
    END SELECT
    q t
    t = "a"
    PRINT "after store: ["; t; "]"; LEN(t); ASC(t + "x", 2)
    t = "much longer text"
    PRINT "after long store: ["; t; "]"; LEN(t)
END SUB

SUB q (v AS STRING)
    PRINT "q: ["; v; "]"; LEN(v)
END SUB

SUB r (t$4)
    PRINT "r: ["; t$4; "]"; LEN(t$4)
    t$4 = "abcdefgh"
END SUB

FUNCTION g$ (t AS STRING * 3)
    g$ = t + "|" + STR$(LEN(t))
END FUNCTION
