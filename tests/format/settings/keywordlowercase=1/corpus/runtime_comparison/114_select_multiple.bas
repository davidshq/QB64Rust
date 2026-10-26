$console:only
' Runtime comparison: SELECT CASE multiple CASE
dim n as long
n = 1
select case n
    case 1, 2, 3
        print "one two three"
    case 4, 5
        print "four five"
    case else
        print "else"
end select
n = 4
select case n
    case 1, 2, 3
        print "one two three"
    case 4, 5
        print "four five"
    case else
        print "else"
end select
end
