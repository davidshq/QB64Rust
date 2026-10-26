$console:only
' Signs the old compiler drops before it reads an expression (study\00 section 5, "Signs the old compiler drops"):
' a + right after a + or a - goes; then two - right after an operator go, so no negation happens there and two
' strings can be joined across them; - -x at the start of an expression stays two negations. In a CONST every
' run of + and - is one sign, - when its number of - is odd.
on error goto h
dim a(5), i as long, r as long
dim i16 as integer, ub as _unsigned _byte, u64 as _unsigned _integer64
x = 3: s$ = "a": a(1) = 7: a(3) = 9
i16 = -32768: ub = 200: u64 = 0: u64 = u64 - 1
print "1."; -x; 2 + x; 2 - x; x + x; x - x; x + -x
print "2."; 2 ^ -x * 3; 2 * -x; 2 ^ -5; -5; x + 5; x - 5
print "3."; 2 - -x; 2 - -x; --x; 2 * -x; 2 ^ -x; 2 ^ x
print "4."; x + not x; x - not x; not x; x + (x); x + f(1, 2)
print "5. "; "a" + s$; s$ + "b"; "a" + s$
s$ = "b" + s$: print "6. "; s$
x = -x: print "7."; x: x = 3
print "8."; 1; -x; a(-1 + 2); f(1, -x)
print "9."; x - x; x + x; x + -x
print "10."; --x; 2 * x; 2 - -x; 2 - x; 2 * -x; 2 + x; -x; --x
print "11."; not x; 2 > x; (--x); a(--1); x and x; x = x; 2 ^ x
print "12."; 2 * i16; --i16; 1 * ub; --ub; 1 * u64; --u64
print "13.";
if x + x then print " yes";
for i = 1 to 2 + 1 step 1: print i;: next
select case x: case 1 + 2: print " three": end select
print "14. a bound written -+3, DIM run twice:";
for r = 1 to 2
    dim b(-3 to 5)
next
print lbound(b)
print "15. a bound written 2 + +3, DIM run twice:";
for r = 1 to 2
    dim c(2 + 3)
next
print ubound(c)
const c1 = -5, c2 = 2 + 3, c3 = 2 - 3, c4 = -3, c5 = + -3, c6 = 2 * + -3, c7 = 2 - -3, c8 = 2 - 3
const c9 = 2 + -3, c10 = 2 + 3, c12 = 2 * -3 + 1, c13 = (-5) * -2, c14$ = "a" + "b", c15 = -5
print "16. CONST:"; c1; c2; c3; c4; c5; c6; c7; c8
print "17. CONST:"; c9; c10; c12; c13; c14$; c15
system
h:
print " [error"; err; "]";
resume next

function f (p, q)
    f = p + q
end function
