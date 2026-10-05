DIM username AS STRING
DIM age AS LONG
OPEN "test.txt" FOR INPUT AS #1
INPUT #1, username, age
CLOSE #1
