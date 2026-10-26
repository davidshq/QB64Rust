$console:only
' Slice (m3-get-put-bload, task 4.3; from verification\v28_e_cmem_arrays): arrays in conventional memory used
' as any array is: ERASE, an
' array listed in COMMON (dynamic to the old compiler), REDIM, a dynamic array, passed to a SUB, LBOUND and
' UBOUND, and the order of the segments.
common shared cm() as integer
dim shared cm(1 to 20) as integer
dim a(1 to 10) as integer, b(1 to 10) as integer, n as integer, i as integer
n = 8
dim dy(1 to n) as long
redim rd(1 to 4) as integer
on error goto handler
' The static ones lie first, in the order of their DIM; the dynamic ones follow from the other end.
print varseg(b(1)) - varseg(a(1)); varseg(cm(1)) > varseg(b(1)); varseg(dy(1)) > varseg(b(1)); varseg(rd(1)) > varseg(b(1))
print varseg(cm(1)) - varseg(dy(1)); varseg(dy(1)) - varseg(rd(1))
print varptr(cm(3)); varptr(dy(3)); varptr(rd(3))
' Values through DEF SEG.
for i = 1 to 10: a(i) = i: next
cm(1) = 258: dy(1) = 65537: rd(2) = 7
def seg = varseg(cm(1)): print peek(varptr(cm(1))); peek(varptr(cm(1)) + 1)
def seg = varseg(dy(1)): print peek(varptr(dy(1))); peek(varptr(dy(1)) + 2)
def seg = varseg(rd(1)): poke varptr(rd(2)), 9: print rd(2)
def seg
' ERASE: a static one is cleared, a dynamic one released; then allocated again.
erase a
print a(3); lbound(a); ubound(a)
erase cm
print "cm erased"
i = cm(1)
redim cm(1 to 5) as integer
cm(5) = 55: print cm(5); ubound(cm)
redim _preserve rd(1 to 8) as integer
print rd(2); ubound(rd)
def seg = varseg(rd(1)): print peek(varptr(rd(2))): def seg
erase dy
redim dy(1 to 3) as long
dy(3) = 3: print dy(3)
' Passed to a SUB.
call sb(a(), rd())
print a(2); rd(1); ubound(rd)
def seg = varseg(rd(1)): print peek(varptr(rd(1))): def seg
system
handler:
print "error"; err
resume next

sub sb (p() as integer, r() as integer)
    p(2) = 22
    erase p
    p(2) = 23
    redim r(1 to 12) as integer
    r(1) = 77
end sub
