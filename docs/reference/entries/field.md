# FIELD

`FIELD` names parts of a random file's record buffer by string variables, for `GET` and `PUT` with no variable:
`FIELD #1, 8 AS nm$, 4 AS men$`.

## Syntax

```text
FIELD [#]filenumber, width AS string$ [, width AS string$]...
```

## Behavior

Each string names the next `width` bytes of the buffer, from its start. After `GET #n, record` each holds its part of
the record; what `LSET` or `RSET` puts into one is what `PUT #n, record` writes.
<!-- src: language/record-files "FIELD"; tests\reference\examples\field_1 -->

An assignment with `=` changes the string and not the buffer. The string stays a field: the next `LSET` to a field of
the file, or `GET` with no variable, puts it back to its part of the buffer. Use `LSET` and `RSET` to set a field.
<!-- src: language/record-files "FIELD when it runs"; Q-014 -->

## Errors

- Widths beyond the record raise error 50, a file that is not open error 52, and one not open `FOR RANDOM` error 54.
  <!-- src: language/record-files "FIELD when it runs" -->
- A target that is no variable-length string is a compile error.
  <!-- src: language/record-files "The forms of FIELD" -->

## Differences

From QuickBASIC 4.5: an assignment with `=` to a field string makes it an ordinary string there, out of the record
buffer. In QB64Rust, as in QB64pe, it stays a field (`DIVERGENCES-QB45.md` Q-014).
<!-- src: Q-014 -->

## Availability

<!-- begin generated: availability -->
Supported. All 14 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:field_1 -->
```qb64
OPEN "field_1.tmp" FOR RANDOM AS #1 LEN = 12
FIELD #1, 8 AS nm$, 4 AS men$
LSET nm$ = "Soult"
LSET men$ = MKL$(30000)
PUT #1, 1
LSET nm$ = "": LSET men$ = ""
GET #1, 1
PRINT "["; nm$; "]"; CVL(men$)
CLOSE #1
KILL "field_1.tmp"
END
```

Output:

```text
[Soult   ] 30000 

Press any key to continue
```
<!-- end generated -->

## See also

[`LSET`](lset.md), [`GET`](get.md), [`PUT`](put.md), [`MKL$`](mki-str.md), [`CVL`](cvi.md).
