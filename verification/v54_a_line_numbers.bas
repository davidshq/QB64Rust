$CONSOLE:ONLY
' Verification (m4-line-numbers-whole-arrays, task 1.1): ERL before any numbered line, after a numbered line,
' after an unnumbered one, for 10.5, 20& and a number beyond LONG, in a SUB and after it, after RESUME; a line
' number in an included file; the same number in the main module and a SUB; GOSUB and RETURN n with numbers;
' ON ERROR GOTO n and RESUME n; ON n GOSUB with numbers.
ON ERROR GOTO 900
PRINT "1. before any numbered line"
ERROR 5
10 PRINT "2. line 10"
ERROR 5
PRINT "3. an unnumbered line after 10"
ERROR 5
10.5 PRINT "4. line 10.5"
ERROR 5
20& PRINT "5. line 20&"
ERROR 5
4294967296 PRINT "6. line 4294967296"
ERROR 5
30 PRINT "7. line 30, then a SUB with line 30 and 40"
s
PRINT "8. back from the SUB"
ERROR 5
'$INCLUDE:'v54_inc/numbered.bi'
PRINT "9. after the included file"
ERROR 5
GOSUB 100
PRINT "10. back from GOSUB 100 (skipped by RETURN 200)"
200 PRINT "11. at 200 after RETURN 200"
ON 2 GOSUB 300, 310
PRINT "12. back from ON 2 GOSUB"
ON ERROR GOTO 920
ERROR 6
PRINT "13. skipped by RESUME 400"
400 PRINT "14. at 400 after RESUME 400"
ERROR 5
PRINT "15. after RESUME NEXT:"; ERL
SYSTEM

100 PRINT "  in GOSUB 100"
RETURN 200
300 PRINT "  at 300 (wrong)": RETURN
310 PRINT "  at 310": RETURN

900 PRINT "  handler 900: ERR"; ERR; "ERL"; ERL
RESUME NEXT

920 PRINT "  handler 920: ERR"; ERR; "ERL"; ERL
ON ERROR GOTO 900
RESUME 400

SUB s
    PRINT "  in s before its line 30: ERL";
    ERROR 5
    30 PRINT "  line 30 of s"
    ERROR 5
    40 PRINT "  line 40 of s"
END SUB
