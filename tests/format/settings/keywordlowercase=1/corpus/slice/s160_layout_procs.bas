$console:only
' Slice (m5b-format-settings, task 1.1): SUBs and FUNCTIONs laid out by hand, for the format oracle under the
' other layout settings (tests\format\settings.list): bodies indented wrong on purpose, blocks inside them, a
' comment after the SUB and FUNCTION lines and their ends, and odd spacing.
DECLARE SUB show (t AS STRING)   ' a DECLARE
dim shared total as long
show "start"
print twice(4); twice(5)
call show("call")
tally 3
print total
print fact&(5)
end

sub show (t as string) ' a SUB
    print "show: "; t
end sub ' its end

function twice% (x as integer) ' a FUNCTION
    twice% = x * 2
end function ' its end

sub tally (n)
    static calls
    calls = calls + 1
    for i = 1 to n
        total = total + i
        if total > 4 then
            exit sub ' leaving early
        end if
    next
end sub

function fact& (n as long)
    select case n
        case is <= 1
            fact& = 1
        case else
            fact& = n * fact&(n - 1)
    end select
end function
