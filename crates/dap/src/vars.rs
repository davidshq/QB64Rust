//! `scopes` and `variables` in BASIC terms (design D7 of `m5-dap-proxy`): each value is read from the debuggee's
//! memory through the symbol file's `c_name`, `reach` and layouts, and shown by `format.rs`. The references the
//! proxy hands out are its own, above [`REF_BASE`]; `lldb-dap`'s, below it, pass through.

use crate::Session;
use crate::format::{Kind, Radix, show, show_string};
use crate::proxy::Proxy;
use crate::symbols::{Reach, Shape, Variable};
use crate::wire::base64_decode;
use serde_json::{Value, json};

/// The proxy's variable references start here. `lldb-dap`'s are below it: small numbers for those of a stop, and
/// from 2^32 for the lasting ones a `repl` evaluate makes. It stays well within the integers a JavaScript client
/// holds exactly (2^53).
pub const REF_BASE: i64 = 1 << 48;

/// The most children shown per level; past it, a note.
pub const MAX_CHILDREN: i64 = 1000;

/// The most bytes of a string's text read.
pub const MAX_TEXT: u64 = 4096;

/// Per dimension of an array: its lower bound, its count of elements, and its multiplier.
pub type Dimensions = Vec<(i64, i64, i64)>;

/// Where an element's value is, from its address.
type MakeLoc = fn(u64) -> Loc;

/// Where a value is.
#[derive(Clone, Debug)]
pub enum Loc {
    /// Its bytes are at the address.
    Bytes(u64),
    /// A `qbs` is at the address.
    Qbs(u64),
    /// A `qbs*` is at the address (a `STRING` member or element).
    QbsPtr(u64),
    /// An array descriptor is at the address.
    Desc(u64),
    /// Not allocated yet: a null pointer, or memory that cannot be read.
    Unallocated,
}

/// A value of the program, as far as the proxy needs to show it.
#[derive(Clone, Debug)]
pub struct Val {
    /// The frame it is read in.
    pub frame: Value,
    pub loc: Loc,
    /// Its type as BASIC spells it (the element type of an array).
    pub ty: String,
    /// The dimensions of an array behind a descriptor.
    pub dims: usize,
    /// The bounds of a member array held in place, and the size of its elements.
    pub in_place: Vec<(i64, i64)>,
    pub elem_size: u64,
    /// A `_DYNAMIC` member array.
    pub dynamic: bool,
    /// The text `evaluate` takes for it: `TOTAL`, `A(2)`, `P.X`.
    pub path: String,
}

pub enum RefKind {
    Locals { frame: Value, procedure: usize },
    Main { frame: Value },
    Val(Val),
}

#[derive(Default)]
pub struct Refs {
    list: Vec<RefKind>,
}

impl Refs {
    pub fn clear(&mut self) {
        self.list.clear();
    }

    pub fn add(&mut self, kind: RefKind) -> i64 {
        self.list.push(kind);
        REF_BASE + i64::try_from(self.list.len()).unwrap_or(0)
    }

    pub fn get(&self, id: i64) -> Option<&RefKind> {
        self.list.get(usize::try_from(id - REF_BASE - 1).ok()?)
    }
}

fn session(p: &Proxy) -> Result<&Session, String> {
    p.state
        .session
        .as_ref()
        .ok_or_else(|| "no program is being debugged".to_string())
}

/// `count` bytes of the debuggee's memory at `addr`, from what this stop has read already when it holds them (a
/// page of an array is read once, design "Risks").
pub fn read(p: &mut Proxy, addr: u64, count: u64) -> Result<Vec<u8>, String> {
    if count == 0 {
        return Ok(Vec::new());
    }
    if let Some(s) = p.state.session.as_ref()
        && let Some((start, block)) = s.memory.iter().find(|(start, b)| {
            addr >= *start && addr - start <= b.len() as u64 && count <= b.len() as u64 - (addr - start)
        })
    {
        let from = usize::try_from(addr - start).unwrap_or(0);
        return Ok(block[from..from + usize::try_from(count).unwrap_or(0)].to_vec());
    }
    let body = p.call(
        "readMemory",
        json!({"memoryReference": format!("0x{addr:x}"), "count": count}),
    )?;
    let bytes = base64_decode(body["data"].as_str().unwrap_or(""))?;
    if (bytes.len() as u64) < count {
        return Err(format!("cannot read {count} bytes at 0x{addr:x}"));
    }
    if let Some(s) = p.state.session.as_mut() {
        s.memory.push((addr, bytes.clone()));
    }
    Ok(bytes)
}

/// Forgets the memory read since the stop, after a write.
pub fn written(p: &mut Proxy) {
    if let Some(s) = p.state.session.as_mut() {
        s.memory.clear();
    }
}

fn u64_at(bytes: &[u8], at: u64) -> u64 {
    let at = usize::try_from(at).unwrap_or(usize::MAX);
    bytes
        .get(at..at.saturating_add(8))
        .map_or(0, |b| u64::from_le_bytes(b.try_into().unwrap_or([0; 8])))
}

fn read_u64(p: &mut Proxy, addr: u64) -> Result<u64, String> {
    Ok(u64_at(&read(p, addr, 8)?, 0))
}

/// The value of a C pointer variable in a frame; 0 for one that cannot be evaluated there.
fn pointer_value(p: &mut Proxy, frame: &Value, c_name: &str) -> u64 {
    p.call(
        "evaluate",
        json!({"expression": format!("(unsigned long long){c_name}"), "frameId": frame, "context": "watch"}),
    )
    .ok()
    .and_then(|b| {
        let r = b["result"].as_str()?.trim().to_string();
        match r.strip_prefix("0x") {
            Some(h) => u64::from_str_radix(h, 16).ok(),
            None => r.parse().ok(),
        }
    })
    .unwrap_or(0)
}

/// A variable of the symbol file, seen from `frame`.
pub fn variable_val(p: &mut Proxy, frame: &Value, v: &Variable) -> Val {
    let ptr = pointer_value(p, frame, &v.c_name);
    let kind = Kind::of(&v.ty);
    let loc = match (ptr, v.reach) {
        (0, _) => Loc::Unallocated,
        (a, Reach::Descriptor) => Loc::Desc(a),
        (a, Reach::String) => Loc::Qbs(a),
        (a, Reach::ParamPointer) if matches!(kind, Kind::String | Kind::Fixed(_)) => Loc::Qbs(a),
        (a, _) => Loc::Bytes(a),
    };
    Val {
        frame: frame.clone(),
        loc,
        ty: v.ty.clone(),
        dims: if v.shape == Shape::Scalar {
            0
        } else {
            v.dims.unwrap_or(1)
        },
        in_place: Vec::new(),
        elem_size: 0,
        dynamic: false,
        path: v.name.clone(),
    }
}

/// The bounds of an array behind a descriptor: per dimension the lower bound, the count and the multiplier; the
/// data pointer. `None` while it is not allocated.
pub fn descriptor(p: &mut Proxy, addr: u64, dims: usize) -> Result<Option<(u64, Dimensions)>, String> {
    let l = &session(p)?.symbols.layouts.array;
    let (slot, data_slot) = (l.slot_size, l.data);
    let slots = l.first_slot + l.slots_each * dims as u64;
    let firsts: Vec<u64> = (0..dims).map(|k| l.dimension_slot(dims, k)).collect();
    let (lb, cn, mu) = (l.lower_bound, l.count, l.multiplier);
    let bytes = read(p, addr, slots * slot)?;
    let at = |s: u64| u64_at(&bytes, s * slot).cast_signed();
    let data = u64_at(&bytes, data_slot * slot);
    let dimensions: Vec<(i64, i64, i64)> = firsts
        .iter()
        .enumerate()
        .map(|(k, &f)| (at(f + lb), at(f + cn), if k == 0 { 1 } else { at(f + mu) }))
        .collect();
    if data == 0 || dimensions.iter().any(|&(_, count, _)| count <= 0) {
        return Ok(None);
    }
    Ok(Some((data, dimensions)))
}

/// How an element of type `ty` is held in an array: its size and where its value is.
fn element(s: &Session, ty: &str) -> Option<(u64, MakeLoc)> {
    Some(match Kind::of(ty) {
        Kind::String => (s.symbols.layouts.pointer_size, Loc::QbsPtr),
        Kind::Float => (s.symbols.layouts.float_element_size, Loc::Bytes),
        Kind::User(name) => (s.symbols.type_named(&name)?.size, Loc::Bytes),
        Kind::Bit | Kind::Mem => return None,
        k => (k.size()?, Loc::Bytes),
    })
}

fn bounds_text(bounds: &[(i64, i64)], ty: &str) -> String {
    let b: Vec<String> = bounds.iter().map(|(lo, hi)| format!("{lo} TO {hi}")).collect();
    format!("({}) AS {ty}", b.join(", "))
}

/// The text of a value, and whether it has children.
pub fn shown(p: &mut Proxy, v: &Val, radix: Radix) -> Result<(String, bool), String> {
    if v.dynamic {
        return Ok(("not shown yet (a _DYNAMIC member array)".into(), false));
    }
    if !v.in_place.is_empty() {
        return Ok((bounds_text(&v.in_place, &v.ty), true));
    }
    let kind = Kind::of(&v.ty);
    match (&v.loc, &kind) {
        (Loc::Unallocated, _) => Ok(("not allocated yet".into(), false)),
        (_, Kind::Bit) => Ok(("not shown yet (_BIT)".into(), false)),
        (_, Kind::Mem) => Ok(("not shown yet (_MEM)".into(), false)),
        (Loc::Desc(a), _) => match descriptor(p, *a, v.dims) {
            Ok(Some((_, dims))) => {
                let bounds: Vec<(i64, i64)> = dims.iter().map(|&(lo, n, _)| (lo, lo.saturating_add(n - 1))).collect();
                Ok((bounds_text(&bounds, &v.ty), true))
            }
            _ => Ok(("not allocated yet".into(), false)),
        },
        (Loc::QbsPtr(a), _) => {
            let q = read_u64(p, *a).unwrap_or(0);
            if q == 0 {
                return Ok(("\"\"".into(), false));
            }
            string_text(p, q).map(|t| (t, false))
        }
        (Loc::Qbs(q), _) => string_text(p, *q).map(|t| (t, false)),
        (Loc::Bytes(_), Kind::User(name)) => Ok((name.clone(), true)),
        (Loc::Bytes(a), k) => {
            let size = k.size().ok_or_else(|| format!("no size for {}", v.ty))?;
            let bytes = read(p, *a, size)?;
            show(k, &bytes, radix)
                .map(|t| (t, false))
                .ok_or_else(|| "cannot be shown".into())
        }
    }
}

/// The text of the `qbs` at `q`, at most [`MAX_TEXT`] bytes of it.
fn string_text(p: &mut Proxy, q: u64) -> Result<String, String> {
    let (chr_at, len_at) = {
        let l = &session(p)?.symbols.layouts;
        (l.qbs_chr, l.qbs_len)
    };
    let head = read(p, q, len_at.max(chr_at) + 8)?;
    let chr = u64_at(&head, chr_at);
    let len = u64_at(&head, len_at) & 0xFFFF_FFFF;
    let bytes = if len == 0 {
        Vec::new()
    } else {
        read(p, chr, len.min(MAX_TEXT))?
    };
    let mut text = show_string(&bytes);
    if len > MAX_TEXT {
        text.push_str(&format!(" … ({len} bytes)"));
    }
    Ok(text)
}

/// The address of the text of the `qbs` at `q`, and its length.
pub fn string_at(p: &mut Proxy, q: u64) -> Result<(u64, u64), String> {
    let (chr_at, len_at) = {
        let l = &session(p)?.symbols.layouts;
        (l.qbs_chr, l.qbs_len)
    };
    let head = read(p, q, len_at.max(chr_at) + 8)?;
    Ok((u64_at(&head, chr_at), u64_at(&head, len_at) & 0xFFFF_FFFF))
}

/// The subscripts of the element at `flat` of an array of these dimensions (lower bound, count, multiplier).
fn subscripts(flat: i64, dims: &[(i64, i64, i64)]) -> Vec<i64> {
    let mut rest = flat;
    let mut out = vec![0; dims.len()];
    for k in (0..dims.len()).rev() {
        let (lo, _, m) = dims[k];
        let m = m.max(1);
        out[k] = lo.saturating_add(rest / m);
        rest %= m;
    }
    out
}

fn element_name(path: &str, subs: &[i64]) -> String {
    let s: Vec<String> = subs.iter().map(i64::to_string).collect();
    format!("{path}({})", s.join(", "))
}

/// The element of an array at the given subscripts.
pub fn element_of(p: &mut Proxy, v: &Val, subs: &[i64]) -> Result<Val, String> {
    let (size, make) = element(session(p)?, &v.ty).ok_or_else(|| format!("elements of {} are not shown yet", v.ty))?;
    let (data, dims, size) = if v.in_place.is_empty() {
        let Loc::Desc(a) = v.loc else {
            return Err(format!("{} is not an array", v.path));
        };
        let (data, dims) = descriptor(p, a, v.dims)?.ok_or_else(|| format!("{} is not allocated yet", v.path))?;
        (data, dims, size)
    } else {
        let Loc::Bytes(a) = v.loc else {
            return Err(format!("{} is not an array", v.path));
        };
        (a, in_place_dims(&v.in_place), v.elem_size)
    };
    if subs.len() != dims.len() {
        return Err(format!("{} has {} dimensions", v.path, dims.len()));
    }
    // The bounds of an array behind a descriptor are the debuggee's memory, garbage before it is set up: the
    // arithmetic on them is checked.
    let unreadable = || format!("{} cannot be read: its bounds are not valid", v.path);
    let mut flat: i64 = 0;
    for (&i, &(lo, n, m)) in subs.iter().zip(&dims) {
        let Some(offset) = i.checked_sub(lo).filter(|&o| o >= 0 && o < n) else {
            return Err(format!(
                "subscript {i} is out of range ({lo} TO {})",
                lo.saturating_add(n.saturating_sub(1))
            ));
        };
        flat = offset
            .checked_mul(m)
            .and_then(|o| flat.checked_add(o))
            .ok_or_else(unreadable)?;
    }
    let at = u64::try_from(flat)
        .ok()
        .and_then(|f| f.checked_mul(size))
        .and_then(|o| data.checked_add(o))
        .ok_or_else(unreadable)?;
    Ok(Val {
        frame: v.frame.clone(),
        loc: make(at),
        ty: v.ty.clone(),
        dims: 0,
        in_place: Vec::new(),
        elem_size: 0,
        dynamic: false,
        path: element_name(&v.path, subs),
    })
}

/// The dimensions of a member array in place: the multiplier of a dimension is the count of those before it.
fn in_place_dims(bounds: &[(i64, i64)]) -> Vec<(i64, i64, i64)> {
    let mut m = 1;
    bounds
        .iter()
        .map(|&(lo, hi)| {
            let d = (lo, hi - lo + 1, m);
            m *= hi - lo + 1;
            d
        })
        .collect()
}

/// A member of a value of a user type.
pub fn member_of(p: &mut Proxy, v: &Val, name: &str) -> Result<Val, String> {
    let Loc::Bytes(a) = v.loc else {
        return Err(format!("{} has no members", v.path));
    };
    let s = session(p)?;
    let t = s
        .symbols
        .type_named(&v.ty)
        .ok_or_else(|| format!("{} has no members", v.path))?;
    let m = t
        .members
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| format!("{} has no member {name}", v.ty))?;
    Ok(member_val(v, a, m))
}

fn member_val(v: &Val, a: u64, m: &crate::symbols::Member) -> Val {
    // A bad address only fails to read.
    let at = a.wrapping_add(m.offset);
    let count: i64 = m.bounds.iter().map(|(lo, hi)| hi - lo + 1).product();
    Val {
        frame: v.frame.clone(),
        loc: if Kind::of(&m.ty) == Kind::String && m.bounds.is_empty() {
            Loc::QbsPtr(at)
        } else {
            Loc::Bytes(at)
        },
        ty: m.ty.clone(),
        dims: 0,
        elem_size: if count > 0 { m.size / count.cast_unsigned() } else { 0 },
        in_place: if m.dynamic { Vec::new() } else { m.bounds.clone() },
        dynamic: m.dynamic,
        path: format!("{}.{}", v.path, m.name),
    }
}

/// The children of a value with children: elements by subscript, or members.
fn children(p: &mut Proxy, v: &Val) -> Result<(Vec<Val>, usize), String> {
    if !v.in_place.is_empty() || matches!(v.loc, Loc::Desc(_)) {
        let dims = if v.in_place.is_empty() {
            let Loc::Desc(a) = v.loc else {
                unreachable!("a descriptor")
            };
            match descriptor(p, a, v.dims)? {
                Some((_, d)) => d,
                None => return Ok((Vec::new(), 0)),
            }
        } else {
            in_place_dims(&v.in_place)
        };
        let total = dims
            .iter()
            .try_fold(1i64, |t, &(_, n, _)| t.checked_mul(n))
            .ok_or_else(|| format!("{} cannot be read: its bounds are not valid", v.path))?;
        let shown = total.min(MAX_CHILDREN);
        // The page in one read; each element's value then comes from it.
        let first = element_of(p, v, &subscripts(0, &dims))?;
        let size = if v.in_place.is_empty() {
            element(session(p)?, &v.ty).map_or(0, |(s, _)| s)
        } else {
            v.elem_size
        };
        if let Loc::Bytes(a) | Loc::QbsPtr(a) = first.loc {
            let _ = read(p, a, size * shown.cast_unsigned());
        }
        let mut out = Vec::new();
        for flat in 0..shown {
            out.push(element_of(p, v, &subscripts(flat, &dims))?);
        }
        return Ok((out, usize::try_from(total - shown).unwrap_or(0)));
    }
    let Loc::Bytes(a) = v.loc else {
        return Ok((Vec::new(), 0));
    };
    let s = session(p)?;
    let Some(t) = s.symbols.type_named(&v.ty) else {
        return Ok((Vec::new(), 0));
    };
    Ok((t.members.iter().map(|m| member_val(v, a, m)).collect(), 0))
}

/// The name a value is listed by under its parent: the last part of its path.
fn short_name(path: &str) -> String {
    path.rsplit_once('.').map_or(path, |(_, last)| last).to_string()
}

/// A variable of a `variables` answer.
pub fn item(p: &mut Proxy, name: &str, v: Val, radix: Radix) -> Value {
    let (value, has_children) = shown(p, &v, radix).unwrap_or_else(|_| ("not allocated yet".into(), false));
    let ty = if v.in_place.is_empty() && !matches!(v.loc, Loc::Desc(_)) {
        v.ty.clone()
    } else {
        value.clone()
    };
    let path = v.path.clone();
    let reference = if has_children {
        p.state.session.as_mut().map_or(0, |s| s.refs.add(RefKind::Val(v)))
    } else {
        0
    };
    json!({"name": name, "value": value, "type": ty, "variablesReference": reference, "evaluateName": path})
}

/// The values a reference lists, by name.
pub fn listed(p: &mut Proxy, reference: i64) -> Result<Vec<(String, Val)>, String> {
    enum What {
        Vars(Value, Vec<Variable>),
        Val(Val),
    }
    let what = {
        let s = session(p)?;
        let vars = |order: Vec<usize>| order.into_iter().map(|i| s.symbols.variables[i].clone()).collect();
        match s
            .refs
            .get(reference)
            .ok_or("that variable is gone: the program has run since")?
        {
            RefKind::Locals { frame, procedure } => {
                let pr = &s.symbols.procedures[*procedure];
                let mut order = pr.params.clone();
                order.extend(
                    s.symbols
                        .variables_of(Some(*procedure))
                        .map(|(i, _)| i)
                        .filter(|i| !pr.params.contains(i)),
                );
                What::Vars(frame.clone(), vars(order))
            }
            RefKind::Main { frame } => What::Vars(
                frame.clone(),
                vars(s.symbols.variables_of(None).map(|(i, _)| i).collect()),
            ),
            RefKind::Val(v) => What::Val(v.clone()),
        }
    };
    match what {
        What::Vars(frame, vars) => Ok(vars
            .into_iter()
            .map(|var| (var.name.clone(), variable_val(p, &frame, &var)))
            .collect()),
        What::Val(v) => {
            let (kids, more) = children(p, &v)?;
            let mut out: Vec<(String, Val)> = kids.into_iter().map(|k| (short_name(&k.path), k)).collect();
            if more > 0 {
                out.push((
                    format!("… {more} more"),
                    Val {
                        loc: Loc::Unallocated,
                        dynamic: true,
                        ..v
                    },
                ));
            }
            Ok(out)
        }
    }
}

pub fn scopes(p: &mut Proxy, m: Value) -> Result<(), String> {
    let frame = m["arguments"]["frameId"].clone();
    let Some(s) = p.state.session.as_mut() else {
        return p.forward(m);
    };
    let Some(&procedure) = frame.as_i64().and_then(|f| s.frames.get(&f)) else {
        return p.respond(&m, Ok(json!({"scopes": []})));
    };
    let locals = match procedure {
        Some(pr) => s.refs.add(RefKind::Locals {
            frame: frame.clone(),
            procedure: pr,
        }),
        None => 0,
    };
    let main = s.refs.add(RefKind::Main { frame });
    p.respond(
        &m,
        Ok(json!({"scopes": [
            {"name": "Locals", "presentationHint": "locals", "variablesReference": locals, "expensive": false},
            {"name": "Main module", "variablesReference": main, "expensive": false},
        ]})),
    )
}

pub fn variables(p: &mut Proxy, m: Value) -> Result<(), String> {
    let reference = m["arguments"]["variablesReference"].as_i64().unwrap_or(0);
    if reference <= REF_BASE {
        return p.forward(m);
    }
    let radix = if m["arguments"]["format"]["hex"] == true {
        Radix::Hex
    } else {
        Radix::Decimal
    };
    let listed = match listed(p, reference) {
        Ok(l) => l,
        Err(e) => return p.respond(&m, Err(e)),
    };
    let mut items = Vec::new();
    for (name, v) in listed {
        if name.starts_with('…') {
            items.push(json!({"name": name, "value": "not listed: more than 1000", "variablesReference": 0}));
        } else {
            items.push(item(p, &name, v, radix));
        }
    }
    p.respond(&m, Ok(json!({"variables": items})))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn subscripts_from_flat() {
        // DIM a(1 TO 3, 0 TO 1): the first dimension varies fastest, its multiplier 1, the second's 3.
        let dims = [(1, 3, 1), (0, 2, 3)];
        assert_eq!(subscripts(0, &dims), [1, 0]);
        assert_eq!(subscripts(2, &dims), [3, 0]);
        assert_eq!(subscripts(3, &dims), [1, 1]);
        assert_eq!(in_place_dims(&[(1, 3), (0, 1)]), [(1, 3, 1), (0, 2, 3)]);
    }

    #[test]
    fn short_names() {
        assert_eq!(short_name("A(1, 2)"), "A(1, 2)");
        assert_eq!(short_name("P.X"), "X");
        assert_eq!(short_name("MANY(1).P.NM"), "NM");
        assert_eq!(short_name("R.V(2)"), "V(2)");
    }
}
