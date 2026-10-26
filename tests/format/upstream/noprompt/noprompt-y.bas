$Console:Only

' With QB64PE_NOPROMPT=y, the ERROR 100 should exit the program without waiting
' for any input
Error 100
Print "This line should not appear"

System
