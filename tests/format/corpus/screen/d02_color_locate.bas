' COLOR and LOCATE in SCREEN 0, with arguments left out, and the shape of the cursor.
Color 14, 1
Print "yellow on blue"
Color , 2
Print "background only"
Color 12
Print "foreground only"
Color 7, 0, 4
Print "with a border colour"
Color 31, 0
Print "blinking"
Color 7
Locate 10, 20
Print "at 10, 20"
Locate , 40
Print "column 40";
Locate 12
Print "row 12";
Locate 14, 5, 0
Print "cursor off";
Locate , , 1
Locate 16, 5, 1, 3, 5
Print "cursor shape";
Locate , , , 1, 7
Locate 20, 30
System
