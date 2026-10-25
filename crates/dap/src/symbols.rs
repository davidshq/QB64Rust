//! The debug symbol file `<exe>.debug.json` (spec `compiler/debug-info`, version 1), read once at `launch` into
//! plain structs (design D3 of `m5-dap-proxy`). A missing field is an error naming it.

use serde_json::Value;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct File {
    /// As the `#line` directives name it.
    pub path: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Sub,
    Function,
}

#[derive(Debug)]
pub struct Procedure {
    /// In upper case, as the symbol file has it.
    pub name: String,
    pub kind: Kind,
    pub c_name: String,
    pub file: usize,
    pub line: u64,
    pub end_file: usize,
    pub end_line: u64,
    pub params: Vec<usize>,
    pub result: Option<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Storage {
    Main,
    Static,
    Local,
    Param,
    Result,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Shape {
    Scalar,
    Static,
    Dynamic,
}

/// How the value is reached from its C name, which is always a pointer variable.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Reach {
    /// The C name points at the value.
    Pointer,
    /// The C name points at an array descriptor.
    Descriptor,
    /// The C name points at a `qbs`.
    String,
    /// The C name points at the bytes of a user type.
    Bytes,
    /// A parameter: the C name points at the caller's value.
    ParamPointer,
}

#[derive(Clone, Debug)]
pub struct Variable {
    pub name: String,
    /// As BASIC spells it: `LONG`, `_UNSIGNED INTEGER`, `STRING * 4`, a user type's name.
    pub ty: String,
    pub storage: Storage,
    pub procedure: Option<usize>,
    pub shape: Shape,
    pub bounds: Vec<(i64, i64)>,
    pub dims: Option<usize>,
    pub c_name: String,
    pub reach: Reach,
}

#[derive(Debug)]
pub struct Member {
    pub name: String,
    pub ty: String,
    pub offset: u64,
    /// The bytes it takes in the type: every element for a member array.
    pub size: u64,
    pub bounds: Vec<(i64, i64)>,
    pub dynamic: bool,
}

#[derive(Debug)]
pub struct Type {
    pub name: String,
    pub size: u64,
    pub members: Vec<Member>,
}

/// The descriptor of an array: slots of `slot_size` bytes; the data pointer and the flags at their slots; per
/// dimension `k` of `n`, from slot `first_slot + slots_each * (n - 1 - k)`, the lower bound, the count and the
/// multiplier at their offsets.
#[derive(Debug)]
pub struct ArrayLayout {
    pub slot_size: u64,
    pub data: u64,
    pub flags: u64,
    pub allocated: u64,
    pub first_slot: u64,
    pub slots_each: u64,
    pub lower_bound: u64,
    pub count: u64,
    pub multiplier: u64,
}

impl ArrayLayout {
    /// The first slot of dimension `k` (0 for the first) of an array of `n` dimensions.
    pub fn dimension_slot(&self, n: usize, k: usize) -> u64 {
        self.first_slot + self.slots_each * (n - 1 - k) as u64
    }
}

#[derive(Debug)]
pub struct Layouts {
    pub pointer_size: u64,
    pub array: ArrayLayout,
    pub qbs_chr: u64,
    pub qbs_len: u64,
    pub qbs_len_size: u64,
    pub float_element_size: u64,
}

#[derive(Debug)]
pub struct Symbols {
    pub files: Vec<File>,
    pub main_c_name: String,
    pub main_file: usize,
    pub procedures: Vec<Procedure>,
    pub variables: Vec<Variable>,
    pub types: Vec<Type>,
    pub layouts: Layouts,
    by_c_name: HashMap<String, usize>,
}

fn field<'a>(v: &'a Value, name: &str, at: &str) -> Result<&'a Value, String> {
    v.get(name)
        .filter(|f| !f.is_null() || name == "procedure" || name == "result" || name == "dims")
        .ok_or_else(|| format!("the debug symbol file has no `{name}` in {at}"))
}

fn text(v: &Value, name: &str, at: &str) -> Result<String, String> {
    field(v, name, at)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| format!("`{name}` in {at} of the debug symbol file is not a string"))
}

fn number(v: &Value, name: &str, at: &str) -> Result<u64, String> {
    field(v, name, at)?
        .as_u64()
        .ok_or_else(|| format!("`{name}` in {at} of the debug symbol file is not a number"))
}

fn index(v: &Value, name: &str, at: &str) -> Result<usize, String> {
    usize::try_from(number(v, name, at)?).map_err(|e| e.to_string())
}

fn optional_index(v: &Value, name: &str, at: &str) -> Result<Option<usize>, String> {
    match field(v, name, at)? {
        Value::Null => Ok(None),
        _ => index(v, name, at).map(Some),
    }
}

fn list<'a>(v: &'a Value, name: &str, at: &str) -> Result<&'a Vec<Value>, String> {
    field(v, name, at)?
        .as_array()
        .ok_or_else(|| format!("`{name}` in {at} of the debug symbol file is not a list"))
}

fn bounds(v: &Value, at: &str) -> Result<Vec<(i64, i64)>, String> {
    list(v, "bounds", at)?
        .iter()
        .map(|b| match (b[0].as_i64(), b[1].as_i64()) {
            (Some(lo), Some(hi)) => Ok((lo, hi)),
            _ => Err(format!(
                "a bound in {at} of the debug symbol file is not a pair of numbers"
            )),
        })
        .collect()
}

fn indices(v: &Value, name: &str, at: &str) -> Result<Vec<usize>, String> {
    list(v, name, at)?
        .iter()
        .map(|i| {
            i.as_u64()
                .and_then(|i| usize::try_from(i).ok())
                .ok_or_else(|| format!("`{name}` in {at} of the debug symbol file holds a non-index"))
        })
        .collect()
}

impl Symbols {
    /// Reads the file at `path`.
    pub fn read(path: &Path) -> Result<Symbols, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let v: Value = serde_json::from_slice(&bytes).map_err(|e| format!("{} is not JSON: {e}", path.display()))?;
        Symbols::parse(&v)
    }

    pub fn parse(v: &Value) -> Result<Symbols, String> {
        let version = number(v, "version", "the file")?;
        if version != 1 {
            return Err(format!(
                "the debug symbol file is of version {version}; this adapter reads version 1"
            ));
        }
        let files = list(v, "files", "the file")?
            .iter()
            .map(|f| {
                Ok(File {
                    path: text(f, "path", "a file")?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let main = field(v, "main", "the file")?;
        let procedures = list(v, "procedures", "the file")?
            .iter()
            .map(|p| {
                let at = "a procedure";
                Ok(Procedure {
                    name: text(p, "name", at)?,
                    kind: match text(p, "kind", at)?.as_str() {
                        "sub" => Kind::Sub,
                        "function" => Kind::Function,
                        k => return Err(format!("a procedure of kind `{k}` in the debug symbol file")),
                    },
                    c_name: text(p, "c_name", at)?,
                    file: index(p, "file", at)?,
                    line: number(p, "line", at)?,
                    end_file: index(p, "end_file", at)?,
                    end_line: number(p, "end_line", at)?,
                    params: indices(p, "params", at)?,
                    result: optional_index(p, "result", at)?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let variables = list(v, "variables", "the file")?
            .iter()
            .map(|x| {
                let at = "a variable";
                Ok(Variable {
                    name: text(x, "name", at)?,
                    ty: text(x, "type", at)?,
                    storage: match text(x, "storage", at)?.as_str() {
                        "main" => Storage::Main,
                        "static" => Storage::Static,
                        "local" => Storage::Local,
                        "param" => Storage::Param,
                        "result" => Storage::Result,
                        s => return Err(format!("a variable of storage `{s}` in the debug symbol file")),
                    },
                    procedure: optional_index(x, "procedure", at)?,
                    shape: match text(x, "shape", at)?.as_str() {
                        "scalar" => Shape::Scalar,
                        "static" => Shape::Static,
                        "dynamic" => Shape::Dynamic,
                        s => return Err(format!("a variable of shape `{s}` in the debug symbol file")),
                    },
                    bounds: bounds(x, at)?,
                    dims: optional_index(x, "dims", at)?,
                    c_name: text(x, "c_name", at)?,
                    reach: match text(x, "reach", at)?.as_str() {
                        "pointer" => Reach::Pointer,
                        "descriptor" => Reach::Descriptor,
                        "string" => Reach::String,
                        "bytes" => Reach::Bytes,
                        "param-pointer" => Reach::ParamPointer,
                        r => return Err(format!("a variable of reach `{r}` in the debug symbol file")),
                    },
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let types = list(v, "types", "the file")?
            .iter()
            .map(|t| {
                let members = list(t, "members", "a type")?
                    .iter()
                    .map(|m| {
                        let at = "a member";
                        Ok(Member {
                            name: text(m, "name", at)?,
                            ty: text(m, "type", at)?,
                            offset: number(m, "offset", at)?,
                            size: number(m, "size", at)?,
                            bounds: bounds(m, at)?,
                            dynamic: field(m, "dynamic", at)?.as_bool().unwrap_or(false),
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                Ok(Type {
                    name: text(t, "name", "a type")?,
                    size: number(t, "size", "a type")?,
                    members,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let l = field(v, "layouts", "the file")?;
        let a = field(l, "array", "the layouts")?;
        let d = field(a, "dimensions", "the array layout")?;
        let q = field(l, "qbs", "the layouts")?;
        let layouts = Layouts {
            pointer_size: number(l, "pointer_size", "the layouts")?,
            array: ArrayLayout {
                slot_size: number(a, "slot_size", "the array layout")?,
                data: number(a, "data", "the array layout")?,
                flags: number(a, "flags", "the array layout")?,
                allocated: number(field(a, "flag_bits", "the array layout")?, "allocated", "the flag bits")?,
                first_slot: number(d, "first_slot", "the dimensions")?,
                slots_each: number(d, "slots_each", "the dimensions")?,
                lower_bound: number(d, "lower_bound", "the dimensions")?,
                count: number(d, "count", "the dimensions")?,
                multiplier: number(d, "multiplier", "the dimensions")?,
            },
            qbs_chr: number(q, "chr", "the qbs layout")?,
            qbs_len: number(q, "len", "the qbs layout")?,
            qbs_len_size: number(q, "len_size", "the qbs layout")?,
            float_element_size: number(l, "float_element_size", "the layouts")?,
        };
        let by_c_name = procedures
            .iter()
            .enumerate()
            .map(|(i, p)| (p.c_name.clone(), i))
            .collect();
        Ok(Symbols {
            files,
            main_c_name: text(main, "c_name", "main")?,
            main_file: index(main, "file", "main")?,
            procedures,
            variables,
            types,
            layouts,
            by_c_name,
        })
    }

    /// The procedure of a frame's function name as `lldb` gives it (`SUB_SHOW(int*)`, `FUNC_TWICE`): `None` for
    /// the main module and for C++ that is no procedure.
    pub fn procedure_of_function(&self, function: &str) -> Option<usize> {
        let c_name = function.split('(').next().unwrap_or(function).trim();
        self.by_c_name.get(c_name).copied()
    }

    /// Whether a frame's function name is the main module's.
    pub fn is_main(&self, function: &str) -> bool {
        function.split('(').next().unwrap_or(function).trim() == self.main_c_name
    }

    /// The variables of a procedure (or of main storage for `None`), in the file's order.
    pub fn variables_of(&self, procedure: Option<usize>) -> impl Iterator<Item = (usize, &Variable)> {
        self.variables.iter().enumerate().filter(move |(_, v)| match procedure {
            None => v.storage == Storage::Main,
            Some(p) => v.procedure == Some(p),
        })
    }

    pub fn type_named(&self, name: &str) -> Option<&Type> {
        self.types.iter().find(|t| t.name.eq_ignore_ascii_case(name))
    }
}
