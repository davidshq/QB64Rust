$CONSOLE:ONLY
s$ = "Austerlitz, Jena, Wagram"
PRINT INSTR(s$, ",")
PRINT INSTR(12, s$, ",")
PRINT INSTR(s$, "Borodino")
first$ = LEFT$(s$, INSTR(s$, ",") - 1)
PRINT first$
END
