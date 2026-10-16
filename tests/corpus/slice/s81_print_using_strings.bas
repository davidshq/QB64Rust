$CONSOLE:ONLY
' Slice (m3-console-files, task 4.3; from verification\v30_b_using_strings): PRINT USING with strings and with
' several values, 17 numbered lines. The string fields, a value of the wrong kind (error 13, what was built is
' printed with no line end), several values and the format starting again, text after the last value used, a
' format with no field (error 5), the end of the statement (a trailing ; or , of either kind), the format read
' once, TAB and SPC among the values, items before USING, an error in a value (the values after it are skipped),
' and a question mark for PRINT. No line prints a zone: a PRINT comma never ends on a redirected console.
ON ERROR GOTO handler
DIM f AS STRING, x AS STRING * 5, n AS LONG, a(2) AS LONG
PRINT "1: [";: PRINT USING "!"; "hello";: PRINT "] [";: PRINT USING "!"; "";: PRINT "] [";: PRINT USING "&"; "hello";
PRINT "] [";: PRINT USING "&"; "";: PRINT "]"
PRINT "2: [";: PRINT USING "\\"; "hello";: PRINT "] [";: PRINT USING "\ \"; "hello";: PRINT "] [";: PRINT USING "\  \"; "hello";
PRINT "] [";: PRINT USING "\      \"; "hello";: PRINT "] [";: PRINT USING "\ab\"; "hello";: PRINT "]"
x = "ab"
PRINT "3: [";: PRINT USING "a&b"; "hello";: PRINT "] [";: PRINT USING "_!&"; "hello";: PRINT "] [";: PRINT USING "&"; x;
PRINT "] [";: PRINT USING "\      \"; x;: PRINT "] [";: PRINT USING "& &"; x + "!"; MID$("abcdef", 2, 3);: PRINT "]"
PRINT "4: [";: PRINT USING "###"; "hello";: PRINT "] [";: PRINT USING "&"; 5;: PRINT "] [";: PRINT USING "!"; 5;: PRINT "]"
PRINT "5: [";: PRINT USING "### &"; 5; 6;: PRINT "] [";: PRINT USING "& ###"; "a"; "b";: PRINT "] [";
PRINT USING "ab ### cd &"; 5; 6;: PRINT "]"
PRINT "6: [";: PRINT USING "## ## ##"; 1; 2; 3;: PRINT "] [";: PRINT USING "## ## ##"; 1; 2;: PRINT "] [";
PRINT USING "## ## ##"; 1;: PRINT "] [";: PRINT USING "##"; 1; 2; 3;: PRINT "] [";: PRINT USING "a##b##c"; 1; 2; 3;: PRINT "]"
PRINT "7: [";: PRINT USING "& is ## years"; "Ann"; 30; "Bob"; 41;: PRINT "] [";: PRINT USING "& is ## years"; "Ann"; 30; "Bob";
PRINT "] [";: PRINT USING "head ## mid ## tail"; 1;: PRINT "] [";: PRINT USING "head ## mid _# tail"; 1;: PRINT "]"
PRINT "8: [";: PRINT USING "abc"; 1;: PRINT "] [";: PRINT USING "abc"; "x";: PRINT "] [";: PRINT USING "_#"; 1;: PRINT "]"
PRINT "9: ";: PRINT USING "##"; 1
PRINT USING "##"; 1;
PRINT "|";
PRINT USING "##"; 1,
PRINT "|"
PRINT "10: ";: PRINT USING "###"; 1;: PRINT POS(0);: PRINT USING "###"; 1,: PRINT POS(0)
f = "<##>"
PRINT "11: ";: PRINT USING f; 5;: PRINT USING f + f; 5; 6;: PRINT USING (f); 7;: PRINT USING STRING$(3, "#") + ".#"; 5
PRINT "12: ";: PRINT USING once$; 1; 2; 3
n = 7
PRINT "13: ";: PRINT USING "### ### ### &"; n + 1; ABS(-5); (n); LEFT$("abcd", 2)
PRINT "14: ";: PRINT USING "## ##"; 1; TAB(10); 2;: PRINT USING "## ##"; 1; SPC(3); 2;: PRINT USING "##"; TAB(40); 1
PRINT "15: ";: PRINT "a"; USING "##"; 5;: PRINT 1; 2; USING "##"; 5; 6;: PRINT "a"; "b" USING "##"; 5;
PRINT TAB(60); USING "##"; 5;: PRINT ; USING "##"; 5
PRINT "16: ";: PRINT USING "## ## ##"; 1; a(9); 3: PRINT "| ";: PRINT USING "## & ##"; 1; MID$("abc", 0); 3: PRINT "| ";
PRINT USING "## ##"; 1; ASC(""): PRINT "|"
PRINT "17: ";: ? USING "##"; 5;: ? USING "&"; " and a string"
SYSTEM
handler:
PRINT "<error"; ERR; ">";
RESUME NEXT
FUNCTION once$
    PRINT "(format)";
    once$ = "#"
END FUNCTION
