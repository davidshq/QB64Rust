//! Every entry of the built-in table compiles (design D10 of `m4-builtins-remaining`; spec `language/builtin-functions`,
//! "Every function of the table compiles"): for each function and statement entry that is no stub, one line is
//! built from its template with arguments of its slot types, and the front end gives it no "not supported yet".
//! Other errors are allowed (a line built from a template alone may miss a rule the old compiler has), a mark is
//! not: it is what this test guards. An entry whose line needs a place of a kind the template does not say (an
//! array for `_WAVE`, a `_MEM` block for `_MEMFREE`) gets its line from [`BY_HAND`]; a name a later step takes is
//! in [`LEFT_MARKED`] with its reason, so that list can only shrink.

use qb64rust_base::SourceMap;
use qb64rust_builtins::template::{Atom, Item};
use qb64rust_builtins::{BUILTINS, Builtin, Kind};
use qb64rust_sema::check;
use qb64rust_syntax::{NoLoader, parse_settled};

/// The lines of the entries whose slots are places of a kind, by call name (a name with two entries is keyed by
/// its call name too).
const BY_HAND: &[(&str, &str)] = &[
    ("sub__wave", "_WAVE n, a8(), 2"),
    ("sub__sndrawbatch", "_SNDRAWBATCH a4(), 2"),
    ("sub__memfree", "_MEMFREE m"),
    ("sub__memcopy", "_MEMCOPY m, 0, 1 TO m, 0"),
    ("func__memget", "n = _MEMGET(m, m.OFFSET, LONG)"),
    ("func__mem", "m = _MEM(n)"),
    ("func__memnew", "m = _MEMNEW(4)"),
    ("func__memimage", "m = _MEMIMAGE(0)"),
    ("func__memsound", "m = _MEMSOUND(n, 0)"),
    ("func__memexists", "n = _MEMEXISTS(m)"),
    ("sub_graphics_get", "GET (0, 0)-(1, 1), a4()"),
    ("sub_graphics_put", "PUT (0, 0), a4()"),
    ("sub_get", "GET #1, , n"),
    ("sub_put", "PUT #1, , n"),
    ("func__embedded", "s = _EMBEDDED$(\"h\")"),
    ("sub_timer", "TIMER ON"),
    ("sub_key", "KEY(1) ON"),
    ("sub_strig", "STRIG(0) ON"),
    ("sub_clear", "CLEAR"),
    ("sub_open", "OPEN s FOR OUTPUT AS #1"),
    ("sub_name", "NAME s AS t"),
    ("sub_seek", "SEEK #1, 1"),
    ("sub_lock", "LOCK #1"),
    ("sub_unlock", "UNLOCK #1"),
    ("sub_line", "LINE (0, 0)-(1, 1)"),
    ("sub_circle", "CIRCLE (0, 0), 1"),
    ("sub_paint", "PAINT (0, 0), 1"),
    ("sub_pset", "PSET (0, 0)"),
    ("sub_preset", "PRESET (0, 0)"),
    (
        "sub__maptriangle",
        "_MAPTRIANGLE (0, 0)-(1, 0)-(0, 1), 0 TO (0, 0)-(1, 0)-(0, 1), 0",
    ),
    ("sub__putimage", "_PUTIMAGE (0, 0), 0"),
    ("sub_pcopy", "PCOPY 0, 1"),
    ("sub_play", "PLAY s"),
    ("sub_sound", "SOUND 440, 1"),
    ("sub_bload", "BLOAD s, 0"),
    ("sub_bsave", "BSAVE s, 0, 1"),
    ("sub_randomize", "RANDOMIZE 1"),
    ("sub_shell", "SHELL s"),
    ("sub_chain", "CHAIN s"),
    ("sub_draw", "DRAW s"),
    ("sub__printstring", "_PRINTSTRING (0, 0), s"),
    ("sub__mapunicode", "_MAPUNICODE 65 TO 66"),
    ("sub__saveimage", "_SAVEIMAGE s"),
    ("sub__screenmove", "_SCREENMOVE 1, 2"),
    ("sub__resize", "_RESIZE ON"),
    ("sub__console", "_CONSOLE ON"),
    ("sub__controlchr", "_CONTROLCHR ON"),
    ("sub__fullscreen", "_FULLSCREEN"),
    ("sub__keyclear", "_KEYCLEAR"),
    ("sub__echo", "_ECHO s"),
    ("sub__dest", "_DEST 0"),
    ("sub__source", "_SOURCE 0"),
    ("sub__blend", "_BLEND"),
    ("sub__dontblend", "_DONTBLEND"),
    ("sub__clearcolor", "_CLEARCOLOR 0"),
    ("sub__setalpha", "_SETALPHA 0"),
    ("sub__palettecolor", "_PALETTECOLOR 1, 2"),
    ("sub__copypalette", "_COPYPALETTE"),
    ("sub__autodisplay", "_AUTODISPLAY"),
    ("sub__font", "_FONT 16"),
    ("sub__freefont", "_FREEFONT 16"),
    ("sub__printmode", "_PRINTMODE _KEEPBACKGROUND"),
    ("sub__mousemove", "_MOUSEMOVE 1, 2"),
    ("sub__depthbuffer", "_DEPTHBUFFER ON"),
    ("sub__clipboardimage", "_CLIPBOARDIMAGE = 0"),
    ("sub__clipboard", "_CLIPBOARD$ = s"),
    ("sub__printimage", "_PRINTIMAGE 0"),
    ("sub__assert", "_ASSERT n"),
    ("sub__midisoundbank", "_MIDISOUNDBANK s"),
    ("sub__sndraw", "_SNDRAW 0"),
    ("sub__sndrawdone", "_SNDRAWDONE"),
    ("sub__sndclose", "_SNDCLOSE n"),
    ("sub__sndplay", "_SNDPLAY n"),
    ("sub__sndplaycopy", "_SNDPLAYCOPY n"),
    ("sub__sndplayfile", "_SNDPLAYFILE s"),
    ("sub__sndpause", "_SNDPAUSE n"),
    ("sub__sndstop", "_SNDSTOP n"),
    ("sub__sndloop", "_SNDLOOP n"),
    ("sub__sndvol", "_SNDVOL n, 1"),
    ("sub__sndbal", "_SNDBAL n, 0"),
    ("sub__sndsetpos", "_SNDSETPOS n, 0"),
    ("sub__sndlimit", "_SNDLIMIT n, 1"),
    ("sub__finishdrop", "_FINISHDROP"),
    ("sub__title", "_TITLE s"),
    ("sub__icon", "_ICON"),
    ("sub__screenhide", "_SCREENHIDE"),
    ("sub__screenshow", "_SCREENSHOW"),
    ("sub__mouseshow", "_MOUSESHOW"),
    ("sub__mousehide", "_MOUSEHIDE"),
    ("sub__mousecursor", "_MOUSECURSOR s"),
    ("sub__windowsizelimit", "_WINDOWSIZELIMIT 1, 2"),
    ("sub__limit", "_LIMIT 1"),
    ("sub__delay", "_DELAY 1"),
    ("sub__display", "_DISPLAY"),
    ("sub__freeimage", "_FREEIMAGE 0"),
    ("sub_sleep", "SLEEP 1"),
    ("sub_beep", "BEEP"),
    ("sub_kill", "KILL s"),
    ("sub_mkdir", "MKDIR s"),
    ("sub_rmdir", "RMDIR s"),
    ("sub_chdir", "CHDIR s"),
    ("sub_environ", "ENVIRON s"),
    ("sub__writefile", "_WRITEFILE s, t"),
    ("sub_poke", "POKE 0, 1"),
    ("sub_files", "FILES"),
    ("sub_date", "DATE$ = s"),
    ("sub_time", "TIME$ = s"),
    ("sub_out", "OUT 1, 2"),
    ("sub_wait", "WAIT 1, 2"),
];

/// The entries a later change takes, with the reason: this list can only shrink.
const LEFT_MARKED: &[(&str, &str)] = &[];

/// A line of the program for an entry: the hand-written one, or the minimal form of its template with an argument
/// of each required slot's type (every optional group left out), `None` when none can be made.
fn line_for(b: &Builtin) -> Option<String> {
    if let Some((_, line)) = BY_HAND.iter().find(|(c, _)| *c == b.callname) {
        return Some(line.to_string());
    }
    let name = format!("{}{}", b.name, b.musthave.unwrap_or(""));
    let arg = |t: &str| match t {
        "STRING" => "s",
        "SINGLE" => "sg",
        "DOUBLE" => "d",
        "_FLOAT" => "fl",
        "INTEGER" => "i",
        "_INTEGER64" => "q",
        "_UNSIGNED _INTEGER64" => "uq",
        "_UNSIGNED LONG" => "u",
        "_OFFSET" | "_UNSIGNED _OFFSET" => "o",
        _ => "n",
    };
    match b.kind {
        Kind::Function => {
            let required = match b.optional {
                Some(opt) => opt.iter().filter(|&&o| !o).count(),
                None => b.arg_types.len(),
            };
            let args: Vec<&str> = b.arg_types.iter().take(required).map(|t| arg(t)).collect();
            let target = match b.ret {
                Some("STRING") => "s",
                Some("_MEM") => "m",
                _ => "d",
            };
            if args.is_empty() {
                Some(format!("{target} = {name}"))
            } else {
                Some(format!("{target} = {name}({})", args.join(", ")))
            }
        }
        Kind::Sub => {
            let Some(format) = b.specialformat else {
                let args: Vec<&str> = b.arg_types.iter().map(|t| arg(t)).collect();
                return Some(format!("{name} {}", args.join(", ")).trim_end().to_string());
            };
            let items = qb64rust_builtins::template::parse(format).ok()?;
            let mut types = b.arg_types.iter();
            let mut out = name.clone();
            fn walk(
                items: &[Item],
                out: &mut String,
                types: &mut std::slice::Iter<&str>,
                arg: &dyn Fn(&str) -> &'static str,
            ) {
                for item in items {
                    match item {
                        Item::Arg => {
                            out.push(' ');
                            out.push_str(arg(types.next().copied().unwrap_or("LONG")));
                        }
                        Item::Punct(c) => out.push(char::from(*c)),
                        Item::Choice(alts) => {
                            out.push(' ');
                            for a in &alts[0] {
                                match a {
                                    Atom::Word(w) => {
                                        out.push_str(w);
                                        out.push(' ');
                                    }
                                    Atom::Punct(c) => out.push(char::from(*c)),
                                }
                            }
                        }
                        Item::Optional(_) => {}
                    }
                }
            }
            walk(&items, &mut out, &mut types, &arg);
            Some(out.split_whitespace().collect::<Vec<_>>().join(" ").replace(" ,", ","))
        }
    }
}

#[test]
fn every_entry_of_the_table_compiles_without_a_mark() {
    let mut marked = Vec::new();
    for b in BUILTINS.iter() {
        if b.callname == "sub_stub" || b.callname.is_empty() && b.kind == Kind::Sub {
            continue;
        }
        if LEFT_MARKED.iter().any(|(c, _)| *c == b.callname) {
            continue;
        }
        let Some(line) = line_for(b) else {
            continue;
        };
        let text = format!(
            "$CONSOLE:ONLY\nDIM s AS STRING, t AS STRING, n AS LONG, i AS INTEGER, d AS DOUBLE, q AS _INTEGER64, sg AS SINGLE\n\
             DIM fl AS _FLOAT, u AS _UNSIGNED LONG, uq AS _UNSIGNED _INTEGER64, o AS _OFFSET, m AS _MEM\n\
             DIM a4(3) AS LONG, a8(3) AS _BYTE\n{line}\nEND\n"
        );
        let mut map = SourceMap::new();
        let file = map.add("t.bas", text.into_bytes());
        let parsed = parse_settled(&mut map, file, &mut NoLoader);
        let (_, diags) = check(&map, &parsed);
        let parse_diags = parsed.diagnostics();
        let all: Vec<&qb64rust_base::Diagnostic> = parse_diags.list().iter().chain(diags.list().iter()).collect();
        if all.iter().any(|d| d.unsupported) {
            let messages: Vec<String> = all
                .iter()
                .map(|d| {
                    format!(
                        "{}{}",
                        if d.unsupported { "not supported yet: " } else { "" },
                        d.message
                    )
                })
                .collect();
            marked.push(format!("{} ({}): {line}: {}", b.name, b.callname, messages.join(" | ")));
        }
    }
    assert!(
        marked.is_empty(),
        "entries marked \"not supported yet\":\n{}",
        marked.join("\n")
    );
}

/// The hand-written lines name entries that exist, so a renamed entry is noticed.
#[test]
fn by_hand_names_entries_of_the_table() {
    for (callname, _) in BY_HAND {
        assert!(
            BUILTINS.iter().any(|b| b.callname == *callname),
            "{callname} is no call name of the table"
        );
    }
}
