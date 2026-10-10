# Glossary

Plain-language definitions of the compiler terms used in this project. Each entry has a simple explanation first,
then a short note on what the term means in QB64Rust.

## Compiler pipeline

### Compiler

**Simply:** a translator. You write instructions in a language people find easy (BASIC), and the compiler turns
them into a language the computer can run. It's like translating a recipe from English into a language only the
kitchen robot understands.

**Here:** `qb64rust` is the new compiler. It reads a `.bas` program and produces an executable, in several steps
(below), each one in its own crate under `crates\`:

```
source → parsing (syntax) → semantic analysis (sema) → IR (ir) → codegen (codegen-cpp) → C++ compiler → .exe
                                         driver runs every step in order
```

The old compiler, `qb64pe.exe`, is the one this project replaces and tests against.

### Front end / back end

**Simply:** the two halves of a translator's job. The front end reads and understands the original. The back end
writes the translation. You can swap the back end to translate into another language without relearning how to
read the original.

**Here:** the front end is `syntax` and `sema`, which understand the BASIC program and find its errors. The back
end is `ir` and `codegen-cpp`, which produce C++. The language server only needs the front end.

### Lexer / token

**Simply:** before you can understand a sentence, you split it into words. A lexer does that for a program. Each
"word" is a token: a keyword such as `PRINT`, a name such as `total`, a number, a string, or a symbol such as `+`.

**Here:** the lexer in the `syntax` crate. It works on raw bytes, never on Rust strings, because QB64 source files
are not always valid UTF-8 (many use the old DOS character set, CP437).

### Parsing

**Simply:** checking the grammar of a sentence and working out its parts. In "The cat sat on the mat", parsing
finds the subject, the verb and the rest. It doesn't care whether the sentence makes sense, only whether it's put
together correctly. "The mat sat on the cat" parses just as well.

**Here:** the `syntax` crate. It takes the lexer's tokens and builds a syntax tree (next entry). Grammar errors,
such as an `IF` with no `END IF`, are reported here.

### Syntax tree / lossless tree

**Simply:** a family-tree drawing of a program. At the top is the whole program, below it each statement, and
below those the pieces of each statement, down to single tokens. It shows which parts belong together.

**Here:** the parser's output. Ours is *lossless*: it keeps every space, comment and line ending, so printing the
tree gives back the original file byte for byte. A formatter or editor features can then work from the tree
without losing anything the user wrote.

### Semantic analysis

**Simply:** checking whether a grammatically correct sentence actually makes sense. "The mat sat on the cat"
has correct grammar but makes no sense. Semantic analysis is the step that notices this, by asking things like
"does this thing exist?" and "can this be done to it?"

**Here:** questions such as: Is this variable declared? Is `x` a number or a string here? Can you add a string to
a number? Does this `GOTO` label exist? Does this `SUB` get the right number of arguments? A program can parse
fine and still fail here.

### sema

**Simply:** short for "semantic analysis". It's the name of the part of the compiler that does it.

**Here:** the `qb64rust-sema` crate (`crates\sema\`). It takes the syntax tree and works out names, scopes, types
and constant values. It produces a typed tree and a symbol table (both below), and reports errors such as "type
mismatch". For cases nobody has yet checked against the old compiler, it reports "not supported yet" instead of
guessing.

### Scope

**Simply:** where a name means something. In your house, "the kitchen" is your kitchen; at a friend's house, the
same words mean theirs. A scope is the "house" in which a name refers to one particular thing.

**Here:** BASIC has the main program's scope and one scope per `SUB` or `FUNCTION`. A variable inside a `SUB` is
separate from a main-program variable with the same name, unless it is declared `SHARED`. `sema` works out which
scope each name belongs to.

### Typed tree

**Simply:** the syntax tree again, but with a label on every part saying what kind of value it is: a whole
number, a decimal number, a string and so on.

**Here:** `sema`'s main output. In `a% + 1.5`, every part gets a type: `a%` is INTEGER, `1.5` is SINGLE, and the
sum is computed as SINGLE. The IR is built from this tree.

### Held type / believed type

**Simply:** what a box really holds, and what its label says. Usually they agree; when they don't, what you get
out depends on whether you open the box or read the label.

**Here:** the old compiler writes C++, and C++ has its own rules for the type of a sum or a literal. The *held*
type is the C++ type a value really has when the program runs; the *believed* type is what the old compiler thinks
it is, which decides how it treats the value next (which conversion, which `PRINT`). They differ, for example, for
`-2147483648&`: its digits `2147483648` are too big for a 32-bit C++ number, so it is held as a 64-bit one, but
believed LONG because of the `&`. `sema` tracks both (`crates\sema\src\check\ops.rs` `held`, design D4 and D7 of
`m2-numeric-types`), so programs compute exactly what the old compiler's programs compute.

### Symbol table

**Simply:** a phone book of everything that has a name in the program. For each name it says where it was
created and every place it is used.

**Here:** kept by `sema` (`crates\sema\src\symbols.rs`) for variables, procedures, labels and constants. The
language server uses it to answer "go to definition" and "find all references".

### Constant folding

**Simply:** doing the sums you can do in advance. If a recipe says "add 2 + 3 eggs", you just write "5 eggs" on
the shopping list.

**Here:** `sema` works out expressions that only involve constants, such as `CONST SIZE = 4 * 8`, at compile
time, using the same rules as the old compiler (including how values wrap when they get too big).

### Lowering

**Simply:** spelling one big instruction out as several basic ones. "Wash the dishes" becomes "fill the sink, add
soap, scrub each plate, rinse, dry". The result is longer and more detailed, but each step is easier to carry
out. It's called *lowering* because the new form is closer to how the machine works ("lower level") and further
from how people write.

**Here:** turning the typed tree into IR. A `FOR` loop, for example, becomes a set of plain steps: set the counter,
test it, run the body, add the step, go back to the test.

### IR (intermediate representation)

**Simply:** a halfway language that the compiler invents for itself. Before translating a book from English to
Japanese, you might first write a very plain, simple outline of it. That outline is easier to translate than the
original, and the same outline could also be translated into French. The IR is that outline.

**Here:** the `ir` crate. It turns the typed tree into a simple, step-by-step description of the program
(procedures, variables, how each argument is passed, error handlers). It doesn't mention C++ or the runtime
library (libqb) at all, so the output language could change later without the earlier steps changing.

### Codegen (code generation)

**Simply:** the step that writes the final translation. The compiler has understood the program and outlined it
(the IR); codegen now writes it out in the target language.

**Here:** the `codegen-cpp` crate. It turns the IR into C++ source files (`main0.txt`, `global.txt`, ...), the
same pieces the old compiler writes. A C++ compiler then turns those into the executable (see Linking).

### Driver

**Simply:** the conductor of an orchestra. It doesn't play any instrument itself; it tells each part when to
play, in the right order, and makes sure the result comes out at the end.

**Here:** the `driver` crate, which builds the `qb64rust` program you run. It reads the command line, loads the
source file, runs parsing, sema, IR and codegen in turn, prints any errors, and then calls the C++ build that
produces the `.exe`.

### Diagnostic

**Simply:** a note from the compiler saying what is wrong and exactly where: which file, which line, which
column. Like a teacher's red pen mark in the margin, with an explanation.

**Here:** the `Diagnostic` type in the `base` crate. Unlike the old compiler, which stops at the first error and
gives only a line, the new one gives columns and can report several errors per run.

### Error recovery

**Simply:** when you find a mistake, you note it and keep reading, instead of giving up on the whole essay. That
way you can report all the mistakes at once.

**Here:** after an error, the compiler skips to the next statement and carries on. It reports at most one error
per statement and at most 100 per run, so one mistake doesn't cause a flood of follow-on errors.

## Runtime and build

### Runtime / libqb

**Simply:** the toolbox a finished program carries with it. Your program says "draw a circle"; the toolbox knows
how to actually put pixels on the screen. The compiler doesn't write that code anew for every program; it calls
the toolbox.

**Here:** libqb is QB64pe's runtime library, written in C++. It does the real work behind `PRINT`, `INPUT`,
graphics, sound, files and so on. For now the build uses the copy in the `..\QB64pe` reference clone; at
milestone M3 a copy comes into this repo.

### Linking

**Simply:** binding the pages of a book together. The program's own code and the runtime's code are compiled
separately; linking joins them into one finished `.exe`.

**Here:** after codegen, a C++ compiler compiles the generated code, and the linker joins it with libqb's
compiled objects to make the executable.

### ABI (application binary interface)

**Simply:** the agreement on how two pieces of compiled code talk to each other: which values go where, in which
order, and who cleans up afterwards. Like agreeing that letters always have the address in the top-right corner.

**Here:** the generated C++ must call libqb exactly the way libqb expects. The IR is deliberately "ABI-neutral":
it knows nothing about these details. Only `codegen-cpp` does, so the agreement lives in one place.

### Overflow / wrapping

**Simply:** a car's mileage counter that only has room for six digits. After 999,999 it rolls over to 000,000.
Numbers in a computer have a fixed size too; going past the largest value is an overflow, and rolling round to
the smallest value is called wrapping.

**Here:** a BASIC `LONG` holds up to 2,147,483,647. Adding 1 to that wraps to -2,147,483,648, matching what the
old compiler's normal build does. This is a recorded project decision, and it applies to `_INTEGER64` too, both
in the running program and in constant folding.

## Testing

### Differential testing

**Simply:** checking a new calculator by typing the same sums into it and into an old one you trust, and comparing
the answers. You don't need to know the right answer to every sum; any difference is a bug in one of them.

**Here:** `crates\difftest` writes BASIC programs that try every operator on every pair of numeric types, with
extreme and random values, as variables and as literals (`tests\differential`). Each program's output is recorded
once from `qb64pe.exe`; tier 2 compiles the same program with `qb64rust` and requires the same output. Only cases
that crash the old compiler's program or fail to compile are left out (each program's header names them); any
other difference is fixed in the new compiler or becomes a decided row in `DIVERGENCES.md`.

## Editor tooling

### Language server (LSP)

**Simply:** a helper that sits beside your editor and understands your program while you type. The editor asks
it questions ("is there an error here?", "where is this defined?") and shows the answers as red squiggles,
outlines and jumps. LSP (Language Server Protocol) is the common language editors use to ask those questions, so
one helper works in many editors.

**Here:** planned for the VS Code extension in `vscode\`, built on the compiler's front end. Its first version
gives syntax errors, an outline, folding, and go to definition for procedures and labels. Until then the
extension gets its errors by running the old compiler.

### Crate / workspace

**Simply:** a crate is one box of Rust code with a label on it. A workspace is the shelf that holds several boxes
that are built together.

**Here:** the repo root is a Rust workspace (`Cargo.toml`) with one crate per compiler stage under `crates\`:
`base`, `syntax`, `builtins`, `sema`, `ir`, `codegen-cpp` and `driver`. Each crate may only use the ones below it
in the pipeline, so the stages can't get tangled together (`crates\README.md`).
