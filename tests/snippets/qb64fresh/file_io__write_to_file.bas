OPEN "test.txt" FOR OUTPUT AS #1
WRITE #1, "name", 42, 3.14
CLOSE #1
