$console:only
' Arrays used without DIM (spec language/arrays-and-types, "Implicit arrays"): in the main module a static array,
' OPTION BASE to 10 per index written, of the suffix's type or the default type; LBOUND and UBOUND of a name that
' is no array make one; the names that stay free; ERASE and REDIM of one; below $DYNAMIC a dynamic array that the
' statement naming it first allocates; in a SUB or FUNCTION an array that nothing allocates (every use raises
' error 9). Error cases go to a handler that prints ERR and resumes next.
on error goto h
dim i as long
z(10) = 4
print "1. z(10) = 4: z(10), LBOUND, UBOUND"; z(10); lbound(z); ubound(z)
print "2. z(11) = 1, z(-1):";
z(11) = 1
print z(-1)
m(2, 3) = 5
print "3. two indexes: UBOUND 1, UBOUND 2, LBOUND 2, m(2, 3), m(10, 10)"; ubound(m, 1); ubound(m, 2); lbound(m, 2); m(2, 3); m(10, 10)
t(1, 2, 3) = 1
print "4. three indexes: UBOUND 3, t(1, 2, 3), t(10, 10, 10)"; ubound(t, 3); t(1, 2, 3); t(10, 10, 10)
s$(2) = "ab": i%(3) = 2.5: d#(1) = 1# / 3: q&&(10) = 5000000000: u~%%(4) = 200: b`(5) = 1
print "5. suffixes: "; s$(2); ubound(s$); i%(3); d#(1); q&&(10); u~%%(4); b`(5); len(i%(3)); len(d#(1))
print "6. the first use is a read:"; r(4); ubound(r); "["; rs$(1); "]"
print "7. LBOUND(lb) first, then lb(10) = 3:"; lbound(lb); ubound(lb);
lb(10) = 3: print lb(10);
print " UBOUND(lb, 2):";
print ubound(lb, 2)
print "8. the first use skipped by GOTO:";
goto sk
w(1) = 1
sk:
print ubound(w); w(1)
print "9. REDIM z(20):";
redim z(20)
print ubound(z); z(10)
dim c(3) as long
c!(7) = 1.5: c(1) = 2
print "10. c!(7) beside DIM c(3) AS LONG: c!(7), c(1), UBOUND(c), UBOUND(c!)"; c!(7); c(1); ubound(c); ubound(c!)
a = 5: a(2) = 3
print "11. a scalar and an array of one name: a, a(2)"; a; a(2)
print "12. an index in a float and in a string of digits:"; z(9.6);
z(val("3")) = 8: print z(3)
dim x as long
x = 7: x(3) = 2.5
print "13. beside DIM x AS LONG: x, x(3), LEN(x(3))"; x; x(3); len(x(3))
const k = 1
k(1) = 2
print "14. beside CONST k: k, k(1)"; k; k(1)
chr(1) = 5: abc(2) = left(3) + 1: __foo(1) = 2
print "15. chr(1), left(3), __foo(1), UBOUND(left):"; chr(1); left(3); __foo(1); ubound(left)
e(1) = 5: es$(2) = "ab"
erase e, es$
print "16. ERASE: e(1), UBOUND(e), es$(2):"; e(1); ubound(e); "["; es$(2); "]"
for i = 0 to 10: f(i) = i * i: next
swap f(2), f(3)
bump f(4)
print "17. filled in a loop, SWAP, an element by reference:"; f(2); f(3); f(4); f(10)
ss$(0) = "x": ss$(10) = ss$(0) + "y"
print "18. strings: "; ss$(10); len(ss$(5))
defint A-Z
option base 1
di(1) = 2.5
print "19. DEFINT A-Z and OPTION BASE 1: di(1), LEN, LBOUND, UBOUND, UBOUND(di%)"; di(1); len(di(1)); lbound(di); ubound(di); ubound(di%)
print "20. di(0):";
print di(0)
dm(1, 1) = 1
print "21. two indexes under base 1:"; lbound(dm, 1); lbound(dm, 2); ubound(dm, 2)
defsng A-Z
option base 0
'$dynamic
dy(3) = 4
print "22. below $DYNAMIC: dy(3), LBOUND, UBOUND"; dy(3); lbound(dy); ubound(dy)
print "23. the first use skipped by GOTO:";
goto sk2
dw(1) = 1
sk2:
print ubound(dw);
dw(1) = 2
print
print "24. the first use run three times:";
for i = 1 to 3
    dl(3) = dl(3) + 1
next
print dl(3); ubound(dl)
print "25. REDIM of it:";
redim dy(20)
print ubound(dy); dy(3)
print "26. a one-line IF taken, and one not taken:";
if i > 0 then it(1) = 2
if i < 0 then nt(1) = 2
print it(1); ubound(it);
print ubound(nt)
print "27. LBOUND first:";
print lbound(dlb); ubound(dlb)
print "28. two in one statement:";
print da(1) + db(1, 2); ubound(da); ubound(db, 2)
print "29. ERASE of it, then a use:";
erase da
print da(1)
ds$(1) = "dyn"
print "30. strings: "; ds$(1); ubound(ds$)
'$static
st(1) = 6
print "31. below $STATIC again:"; st(1); ubound(st)
redim st(3)
print
print "32. a SUB, called twice:";
plain
plain
print
print "33. a FUNCTION:";
i = fn(1)
print
print "34. beside main arrays, a parameter and a DIM SHARED scalar of the name:";
dim shared g as long
g = 3
names 4
print z(10); g
print "35. ERASE in a SUB:";
wipe
print

system
h:
print " [error"; err; "]";
resume next

sub bump (v)
    v = v + 100
end sub
sub plain
    x(2) = x(2) + 1
    print " x(2):";
    print x(2);
end sub
function fn (n)
    x(2) = 5
    print " UBOUND(x), LBOUND(x):";
    print ubound(x); lbound(x);
    s$(1) = "a"
    print " ["; s$(1); "]";
end function
sub names (n)
    print " z(10):";
    print z(10);
    g(1) = 2.5
    print " g(1):";
    print g(1);
    n(1) = 1
    print " n(1), n:";
    print n(1); n
end sub
sub wipe
    w2(2) = 5
    erase w2
    print " UBOUND(w2):";
    print ubound(w2);
end sub
