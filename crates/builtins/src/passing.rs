//! How the old compiler passes the parts of a `specialformat` template to the libqb entry (`qb64pe.bas`
//! `seperateargs`, the pass rules; `study\00` §5, "Statement calls"): which arguments and choices become C
//! arguments, and which bit of the trailing `passed` mask says that an optional part was written.
//!
//! The rule, as read there:
//!
//! - an argument (`?`) is always a C argument: its value, or `NULL` when left out;
//! - a choice with several alternatives (`{B|BF}`) is a C argument: the number of the alternative written, from
//!   1, or `NULL` when left out;
//! - a choice with one alternative is never a C argument. Outside `[…]` it is mandatory and says nothing
//!   (`{As}`); inside, it may get a flag;
//! - flags are given per optional block (the parts directly inside one `[…]`), blocks of the outermost level
//!   first, then the next level, each level left to right: a block with a several-alternative choice needs none
//!   (the choice's `NULL` says "absent"); otherwise a block with an argument, or with only one-alternative choices,
//!   gets the next bit (1, 2, 4, …), set when the block was written. A block of punctuation alone gets none.
//!
//! The entry has a last `passed` argument when any flag was given.

use crate::template::Item;

/// One argument or choice of a template, in template order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Part {
    /// The number of alternatives of a choice; 0 for an argument.
    pub alts: u8,
    /// Inside `[…]`: it may be left out.
    pub optional: bool,
    /// It is a C argument of the call.
    pub passed: bool,
    /// The bit of the `passed` mask set when the part is written; 0 for none.
    pub flag: u32,
}

/// The parts of a template and whether its call ends with the `passed` mask.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Plan {
    pub parts: Vec<Part>,
    pub mask: bool,
}

/// An element as the old compiler lists it: an argument, a choice, or a punctuation character.
struct Element {
    /// The nesting depth of `[…]` around it.
    level: usize,
    /// The level it continues, or the innermost enclosing level already entered when it starts its own.
    entry: usize,
    /// 0 for an argument, the number of alternatives of a choice, 1 for punctuation.
    options: usize,
    dont_pass: bool,
    flag: u32,
    /// False for punctuation.
    is_part: bool,
}

fn flatten(items: &[Item], level: usize, entered: &mut Vec<bool>, out: &mut Vec<Element>) {
    for item in items {
        let options = match item {
            Item::Optional(inner) => {
                if entered.len() <= level + 1 {
                    entered.resize(level + 2, false);
                }
                entered[level + 1] = false;
                flatten(inner, level + 1, entered, out);
                continue;
            }
            Item::Arg => 0,
            Item::Choice(alts) => alts.len(),
            Item::Punct(_) => 1,
        };
        let entry = if entered[level] {
            level
        } else {
            (1..level).rev().find(|&l| entered[l]).unwrap_or(0)
        };
        entered[level] = true;
        let punct = matches!(item, Item::Punct(_));
        out.push(Element {
            level,
            entry,
            options,
            dont_pass: punct,
            flag: 0,
            is_part: !punct,
        });
    }
}

/// The pass plan of a template.
pub fn plan(items: &[Item]) -> Plan {
    let mut els = Vec::new();
    flatten(items, 0, &mut vec![false], &mut els);
    for e in &mut els {
        if e.level == 0 && e.options == 1 {
            e.dont_pass = true;
        }
    }
    let mut next_flag = 1u32;
    for level in 1.. {
        let mut deeper = false;
        // The block being scanned: whether one is open, its first must-pass element and whether that needs a flag,
        // and the elements that share the block's flag.
        let mut open = false;
        let mut must: Option<bool> = None;
        let mut list: Vec<usize> = Vec::new();
        let close = |els: &mut Vec<Element>, must: Option<bool>, list: &[usize], next_flag: &mut u32| {
            // A several-alternative choice in the block needs no flag; the others refer to it.
            if must == Some(false) || list.is_empty() {
                return;
            }
            for &x in list {
                els[x].flag = *next_flag;
            }
            *next_flag *= 2;
        };
        for x in 0..els.len() {
            if els[x].level > level {
                deeper = true;
            }
            if open && els[x].entry < level {
                close(&mut els, must, &list, &mut next_flag);
                open = false;
            }
            if els[x].level == level && els[x].entry < level {
                open = true;
                must = None;
                list.clear();
            }
            if open && els[x].level == level {
                if els[x].options != 1 {
                    let needs_flag = els[x].options == 0;
                    match must {
                        None => must = Some(needs_flag),
                        Some(true) if !needs_flag => must = Some(false),
                        Some(_) => {}
                    }
                    list.push(x);
                } else if !els[x].dont_pass {
                    list.push(x);
                    els[x].dont_pass = true;
                }
            }
        }
        if open {
            close(&mut els, must, &list, &mut next_flag);
        }
        if !deeper {
            break;
        }
    }
    Plan {
        parts: els
            .iter()
            .filter(|e| e.is_part)
            .map(|e| Part {
                alts: u8::try_from(e.options).expect("fewer than 256 alternatives"),
                optional: e.level > 0,
                passed: !e.dont_pass,
                flag: e.flag,
            })
            .collect(),
        mask: next_flag != 1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::template::parse;

    /// Per part: `?` or the number of alternatives, `!` when it is not a C argument, and its flag if any.
    fn shown(template: &str) -> (String, bool) {
        let p = plan(&parse(template).unwrap());
        let parts: Vec<String> = p
            .parts
            .iter()
            .map(|part| {
                let mut s = if part.alts == 0 {
                    "?".to_string()
                } else {
                    part.alts.to_string()
                };
                if !part.passed {
                    s.push('!');
                }
                if part.flag != 0 {
                    s.push_str(&format!("/{}", part.flag));
                }
                s
            })
            .collect();
        (parts.join(" "), p.mask)
    }

    /// Checked against the C++ of `qb64pe -z` (`verification\v22_*`): `sub_open(name,4,NULL,NULL,1,NULL,0)`,
    /// `sub_name(a,b)`, `sub_seek(1,2)`, `sub_randomize(5,1)`, `func_rnd(NULL,0)`.
    #[test]
    fn plans_of_measured_statements() {
        let open = "?[{For Random|For Binary|For Input|For Output|For Append}][{Access Read Write|Access Read|Access Write}][{Shared|Lock Read Write|Lock Read|Lock Write}]{As}[#]?[{Len =}?]";
        assert_eq!(shown(open), ("? 5 3 4 1! ? 1!/1 ?/1".into(), true));
        assert_eq!(shown("?{As}?"), ("? 1! ?".into(), false));
        assert_eq!(shown("[#]?,?"), ("? ?".into(), false));
        assert_eq!(shown("?,[#]?,?[,?]"), ("? ? ? ?/1".into(), true));
        assert_eq!(shown("[[{Using}]?]"), ("1!/2 ?/1".into(), true));
        assert_eq!(shown("[?]"), ("?/1".into(), true));
        assert_eq!(shown("{_Hide}[{_DontWait}][?]"), ("1! 1!/1 ?/2".into(), true));
        assert_eq!(shown("?,?"), ("? ?".into(), false));
    }

    /// `LINE`: the coordinates' block and the second `STEP` are of the outermost level (1, 2), then the first `STEP`
    /// and the colour (4, 8), then the style (16); the box word is a several-alternative choice, so its block has no
    /// flag. Measured: `LINE STEP(1,2)-STEP(3,4), 5, BF, 6` is `sub_line(1,2,3,4,5,2,6,31)`, `LINE -(3,4), 5` ends
    /// `…,8)`, `LINE -STEP(3,4), , B` is `sub_line(NULL,NULL,3,4,NULL,1,NULL,2)`, `LINE (1,2)-(3,4), , , 7` ends `…,17)`.
    #[test]
    fn nested_blocks() {
        let line = "[[{Step}](?,?)]-[{Step}](?,?)[,[?][,[{B|BF}][,?]]]";
        let (parts, mask) = shown(line);
        assert!(mask);
        assert_eq!(parts, "1!/4 ?/1 ?/1 1!/2 ? ? ?/8 2 ?/16");
    }
}
