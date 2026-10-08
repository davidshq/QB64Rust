$CONSOLE:ONLY
' Verification (m2-numeric-types, task 1.6): where the old compiler converts an out-of-range suffixed literal, a
' suffixed CONST out of range and a bit-suffixed literal to the suffix's type: stores into wider variables,
' arguments by value to procedures and built-ins, unary minus, parentheses, operators, CASE items, FOR limits, IF
' conditions, an array bound, HEX$ and FUNCTION results (v21_a_literals, v21_d_const: PRINT converts, + 0 does not).
ON ERROR GOTO h
CONST c~%% = 300, n~& = -1, b`3 = 9
DIM l AS LONG, q AS _INTEGER64, d AS DOUBLE, u AS _UNSIGNED _BYTE
l = 300~%%: q = -1~&: d = 300~%%: u = 300~%%
PRINT "store:"; l; q; d; u
l = c~%%: q = n~&: d = b`3
PRINT "store const:"; l; q; d
l = 9`3: q = -1~`
PRINT "store bit:"; l; q
k% = 40000%: kl& = 40000%
PRINT "store 40000%:"; k%; kl&
showl 300~%%
showl c~%%
showl 9`3
showl 40000%
showd 300~%%
showq -1~&
PRINT "STR$:"; STR$(300~%%); STR$(c~%%); STR$(9`3); STR$(-1~&)
PRINT "ABS:"; ABS(-1~&); ABS(n~&); ABS(300~%%)
PRINT "LEFT$:"; LEN(LEFT$("abcdefghij", 260~%%)); LEN(LEFT$("abcdefghij", c~%%))
PRINT "CHR$:"; ASC(CHR$(321~%%))
PRINT "neg:"; -(300~%%); -c~%%; -(9`3); -(-1~&)
PRINT "paren:"; (300~%%); (c~%%); (-1~&); (9`3)
PRINT "mix:"; 300~%% * 2; c~%% * 2; 300~%% / 2; 9`3 * 2; 300~%% AND 255; NOT 300~%%; 300~%% \ 7
PRINT "mix2:"; 300~%% + 0~%%; 40000% + 0; -1~& + 1~&; -1~& * 1~&
PRINT "&& held 64-bit:"; 2147483647&& + 1; 2147483647& + 1; 65536&& * 65536; 65536& * 65536
SELECT CASE 44
    CASE 300~%%: PRINT "case 300~%%, selector 44: matches"
    CASE ELSE: PRINT "case 300~%%, selector 44: misses"
END SELECT
SELECT CASE 300
    CASE 300~%%: PRINT "case 300~%%, selector 300: matches"
    CASE ELSE: PRINT "case 300~%%, selector 300: misses"
END SELECT
x~%% = 44
SELECT CASE x~%%
    CASE 300~%%: PRINT "case 300~%%, selector x~%% 44: matches"
    CASE ELSE: PRINT "case 300~%%, selector x~%% 44: misses"
END SELECT
SELECT CASE 300~%%
    CASE 44: PRINT "selector 300~%%, case 44: matches"
    CASE 300: PRINT "selector 300~%%, case 300: matches"
    CASE ELSE: PRINT "selector 300~%%: neither"
END SELECT
cnt = 0: FOR i = 1 TO 260~%%: cnt = cnt + 1: NEXT
PRINT "FOR to 260~%%:"; cnt
cnt = 0: FOR i = 1 TO c~%%: cnt = cnt + 1: NEXT
PRINT "FOR to c~%%:"; cnt
cnt = 0: FOR i = 1 TO 9`3: cnt = cnt + 1: NEXT
PRINT "FOR to 9`3:"; cnt
IF 256~%% THEN PRINT "IF 256~%%: true" ELSE PRINT "IF 256~%%: false"
IF 300~%% = 44 THEN PRINT "300~%% = 44: true" ELSE PRINT "300~%% = 44: false"
DIM a(258~%%)
PRINT "UBOUND:"; UBOUND(a)
PRINT "HEX$:"; HEX$(300~%%); " "; HEX$(c~%%); " "; HEX$(9`3); " "; HEX$(40000%)
PRINT "fn:"; f&(300~%%); fu~%%(300); fb(9`3)
SYSTEM

h:
PRINT "[handler"; ERR; "]"
RESUME NEXT

SUB showl (x AS LONG)
    PRINT "showl:"; x
END SUB

SUB showd (x AS DOUBLE)
    PRINT "showd:"; x
END SUB

SUB showq (x AS _INTEGER64)
    PRINT "showq:"; x
END SUB

FUNCTION f& (x AS LONG)
    f& = x
END FUNCTION

FUNCTION fu~%% (x AS LONG)
    fu~%% = x
END FUNCTION

FUNCTION fb (x AS _INTEGER64)
    fb = x
END FUNCTION
