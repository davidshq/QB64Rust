'Test to prove the general auto-include files
'beforefirstline.bi and afterlastline.bm are really included.

$Console:Only

'beforefirstline.bi contains lots of constants,
'we print some to prove the file is included
Print "Prove availability of beforfirstline.bi constants:"
Print "--------------------------------------------------"
Print "Base of natural logarithm:"; _E
Print "Seconds in day...........:"; _SECS_IN_DAY
Print "One Gigabyte.............:"; _ONE_GB
Print

'afterlastline.bm contains some support functions,
'let's call one to prove the file is included
Print "Prove availability of afterlastline.bm functions:"
Print "-------------------------------------------------"
Print "Encoding a URL...........: "; _EncodeURL$("http://w%w.example&.com/")

System

