$console:only
' Slice (m5b-format-settings, task 1.1): blocks of every kind laid out by hand, for the format oracle under the
' other layout settings (tests\format\settings.list). The indentation is written wrong on purpose, the spacing
' is odd, the keywords are in mixed case, and each block line has a comment after it.
type pair ' a record
    a as integer ' first
    b as string * 3
end type ' end of the record
dim p as pair: p.a = 7: p.b = "xyz"
print p.a; p.b
i% = 0
do while i% < 3 ' a loop tested first
    i% = i% + 1
    if i% = 2 then ' a block IF
        print "two"
    elseif i% = 3 then ' its ELSEIF
        print "three"
    else ' its ELSE
        print "one"
    end if ' its END IF
loop ' end of the loop
do ' a loop tested last
    i% = i% - 1
loop until i% <= 0 ' its end
for j = 1 to 3 step 1 ' a FOR
    for k = 1 to j ' a nested FOR
        print j * k;
    next k ' its NEXT
    print
next ' end of the outer FOR
while j > 1 ' a WHILE
    j = j - 1
wend ' its WEND
for n = 1 to 5
    select case n ' a SELECT CASE
        case 1 ' a CASE
            print "a";
        case 2, 3 ' a CASE with two values
            print "b";
        case is > 4 ' CASE IS
            print "c";
        case else ' CASE ELSE
            print "d";
    end select ' end of the SELECT
next n
print
if j = 1 then print "single line" else print "not" ' a single-line IF
if j = 1 then
    print "joined";: print " in a block": i% = 0
end if
for q = 1 to 2: print q;: next: print
10 print "line ten" ' a line number
20 if j = 1 then goto done ' a jump
print "not reached"
done: ' a label
print "done"
again: print "label and statement"
