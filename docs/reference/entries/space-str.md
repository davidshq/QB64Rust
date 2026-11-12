# SPACE$

`SPACE$` gives a string of spaces: `SPACE$(4)` is `"    "`.

## Syntax

<!-- begin generated: syntax -->
```text
SPACE$(count&)
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Arguments

- The number of spaces.

## Result

A STRING.

## Behavior

`SPACE$(n)` is `n` spaces. A count of 0 or less gives `""`, with no error.
<!-- src: language/builtin-functions "String results"; verification\v20_c_string_edges; tests\reference\examples\space-str_1 -->

## Errors

- A string as the count is a compile error: "`SPACE$` needs a number".
  <!-- src: language/builtin-functions "Names and arity" -->

## Availability

<!-- begin generated: availability -->
Supported. The one program that uses it compiles (a call-site program, an example or a probe).
<!-- end generated -->

## Example

<!-- begin generated: example:space-str_1 -->
```qb64
PRINT "["; SPACE$(4); "]"
PRINT "Name"; SPACE$(6); "Men"
PRINT LEN(SPACE$(0))
END
```

Output:

```text
[    ]
Name      Men
 0 

Press any key to continue
```
<!-- end generated -->

## See also

`STRING$`, `SPC`, `LTRIM$`.
