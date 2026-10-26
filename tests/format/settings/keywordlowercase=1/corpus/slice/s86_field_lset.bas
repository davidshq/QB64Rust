$console:only
' Slice (m3-console-files, task 6.2; from verification\v30_j_field, v30_k_field_errors, v30_l_lset_rset): FIELD,
' LSET and RSET, 20 numbered lines. Records written through fields and read back with GET and PUT with no
' variable; what a field string holds after FIELD; a second FIELD on the file (both name the buffer from its first
' byte); = to a field variable (the string changes, the buffer does not, and the string is still a field); GET and
' PUT with a variable beside fields (the buffer is not theirs); a width of 0; after CLOSE; the errors of FIELD
' (widths beyond the record 50, a negative width 5, not open 52, not random 54) and of GET and PUT with no variable
' (record 0: 63, a binary file: 54, ACCESS READ and WRITE: 75); the file number before a target's index before
' its width; elements and members as fields. LSET and RSET on any string: shorter, equal and longer values, an
' empty target, fixed-length strings, elements, members, the target in its own value, an error in the value.
' The files are s86_field_lset.tmp and s86_field_lset_2.tmp and are removed.
on error goto handler
type t
    s as string * 6
    v as string
    n as long
end type
dim f as string, a as string, b as string, c as string, d as string, w as string, k as long, l as long
dim ub as _unsigned _byte, x as string * 8, y as string * 5, e(1 to 3) as string, r as t, q(2) as t
dim fx(2) as string * 4, s as single
f = "s86_field_lset.tmp"
open f for random as #1 len = 8
a = "before"
field #1, 3 as a, 5 as b
print "1:"; len(a); len(b); asc(a); asc(b)
lset a = "xy": lset b = "hello"
print "2: ["; a; "]["; b; "]";: put #1, 1: print lof(1); seek(1)
lset a = "": lset b = ""
print "3: ["; a; "]["; b; "]";: get #1, 1: print "["; a; "]["; b; "]"; seek(1)
rset a = "z": rset b = "ab"
print "4: ["; a; "]["; b; "]";: put #1: print lof(1); seek(1);: put #1, 4: print lof(1); seek(1)
print "5:";: get #1, 2: print " ["; a; "]["; b; "]";: get #1, 3: print asc(a); asc(b); eof(1);: get #1, 9: print asc(a); eof(1); lof(1)
get #1, 1
field #1, 8 as w
print "6: ["; w; "]["; a; "]["; b; "]";: lset w = "12345678": print "["; a; "]["; b; "]";: lset b = "B": print "["; w; "]"
field #1, 2 as c, 2 as d
print "7: ["; c; "]["; d; "]";: field #1, 4 as a: print "["; a; "]"; len(a);: lset w = "abcdefgh": print "["; a; "]["; b; "]["; c; "]"
a = "QQ"
print "8: ["; a; "]"; len(a); "["; w; "]";: lset w = "ABCDEFGH": print "["; a; "]"; len(a);
a = "longer than the field": print len(a);: get #1, 1: print "["; a; "]"; len(a)
b = "": print "9:"; len(b);: lset b = "k": print len(b); "["; w; "]"
lset w = "ABCDEFGH": put #1, 1
print "10:";: l = 0: get #1, 1, l: print l; "["; w; "]";: x = "ijklmnop": put #1, 1, x: print "["; w; "]";: get #1, 1: print "["; w; "]"
field #1, 0 as c, 8 as d
print "11:"; len(c); "["; d; "]";: lset c = "x": print len(c)
close #1
print "12: ["; w; "]"; len(w);: lset w = "closed": print "["; w; "]";: w = "free": print "["; w; "]"
open f for random as #1 len = 8
print "13:"; lof(1); "["; d; "]";: lset d = "again": put #1, 1: field #1, 8 as w: get #1, 1: print asc(w); "["; d; "]"
print "14:";: field #1, 20 as a: print len(a);: field #1, 5 as a, 4 as b, 1 as c: print len(a); len(b); len(c);
field #1, -1 as c: print len(c);: field #1, 2.5 as c: print len(c);: field #1, 3.5 as c: print len(c)
k = 0: s = 2.6
print "15:";: field 1, 4 as c: print len(c);: field #k + 1, s + 2 as c: print len(c);: field (1), 7 as c: print len(c);
field #3, 2 as c: print len(c);: field #0, 2 as c: print len(c);: field #-1, 2 as c: print len(c)
field #1, 2 as e(1), 3 as e(2), 3 as r.v
lset e(1) = "ab": lset e(2) = "cde": lset r.v = "fgh"
field #1, 8 as a
print "16:"; len(e(1)); len(e(2)); len(e(3)); len(r.v); "["; a; "]";: k = 2: field #1, 4 as e(k + 1), 4 as q(k).v: print "["; e(3); "]["; q(2).v; "]";
field #1, 2 as e(4): print
print "17:";: field #fn&, wd& as c: print len(c);: field #fn&, wd& as e(ix&): print len(e(1))
print "18:";: get #1, 0: print seek(1);: put #1, -1: print seek(1);: get #3: put #3: print
close #1
open f for binary as #1: print "19:";: field #1, 2 as c: get #1: put #1: close #1
open "s86_field_lset_2.tmp" for output as #1: field #1, 2 as c: close #1: kill "s86_field_lset_2.tmp"
open f for random access read as #1 len = 8: field #1, 8 as c: get #1, 1: put #1, 1: close #1
open f for random access write as #1 len = 8: field #1, 8 as c: get #1, 1: put #1, 1: close #1
print
open f for binary as #1
print "20:";: for k = 1 to lof(1): get #1, k, ub: print ub;: next: print
close #1: kill f
a = space$(5)
lset a = "ab": print "21: ["; a; "]";: rset a = "ab": print "["; a; "]";: lset a = "abcde": print "["; a; "]";: rset a = "vwxyz": print "["; a; "]"
a = space$(3)
lset a = "abcdef": print "22: ["; a; "]";: rset a = "abcdef": print "["; a; "]";: lset a = "": print "["; a; "]";
a = "": lset a = "abc": print "["; a; "]"; len(a);: rset a = "abc": print len(a)
lset y = "ab": print "23: ["; y; "]";: rset y = "ab": print "["; y; "]";: rset y = "abcdefgh": print "["; y; "]";: lset y = "": print "["; y; "]"
e(2) = ".....": k = 1
lset e(2) = "ab": print "24: ["; e(2); "]";: rset e(k + 1) = "cd": print "["; e(2); "]";: rset fx(1) = "z": print "["; fx(1); "]";: lset e(4) = "ab": print
r.v = "12345": q(2).v = "1234"
lset r.s = "ab": print "25: ["; r.s; "]";: rset r.s = "ab": print "["; r.s; "]";: lset r.v = "ab": print "["; r.v; "]";: rset q(1).s = "ab": rset q(2).v = "a": print "["; q(1).s; "]["; q(2).v; "]"
a = "abcdef": lset a = mid$(a, 3): print "26: ["; a; "]";: a = "abcdef": rset a = left$(a, 2): print "["; a; "]";: lset a = a + "x": print "["; a; "]"
e(1) = "111"
print "27:";: lset e(ix&) = vl$: print " ["; e(1); "]";: a = space$(8): rset a = r.s: print "["; a; "]";: lset y = a: print "["; y; "]"
a = "keep."
print "28:";: lset a = chr$(-1): print "["; a; "]";: if len(a) then lset a = "x" else rset a = "y"
print "["; a; "]"
system
handler:
print " <error"; err; ">";
resume next
function fn&
    print " fn";
    fn& = 1
end function
function wd&
    print " wd";
    wd& = 3
end function
function ix&
    print " ix";
    ix& = 1
end function
function vl$
    print " vl";
    vl$ = "v"
end function
