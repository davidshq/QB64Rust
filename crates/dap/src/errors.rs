//! The text of each runtime error, as libqb gives it (`libqb\internal\c\libqb\src\error_handle.cpp`:
//! `error_code_to_text`, and the critical errors of `error`), for the description of an error stop (design D10 of
//! `m5-dap-proxy`). A test holds the table to that file.

/// `error_code_to_text`'s cases, in its order.
pub const TEXTS: &[(i64, &str)] = &[
    (0, "No error"),
    (1, "NEXT without FOR"),
    (2, "Syntax error"),
    (3, "RETURN without GOSUB"),
    (4, "Out of DATA"),
    (5, "Illegal function call"),
    (6, "Overflow"),
    (7, "Out of memory"),
    (8, "Label not defined"),
    (9, "Subscript out of range"),
    (10, "Duplicate definition"),
    (11, "Division by zero"),
    (12, "Illegal in direct mode"),
    (13, "Type mismatch"),
    (14, "Out of string space"),
    (16, "String formula too complex"),
    (17, "Cannot continue"),
    (18, "Function not defined"),
    (19, "No RESUME"),
    (20, "RESUME without error"),
    (24, "Device timeout"),
    (25, "Device fault"),
    (26, "FOR without NEXT"),
    (27, "Out of paper"),
    (29, "WHILE without WEND"),
    (30, "WEND without WHILE"),
    (33, "Duplicate label"),
    (35, "Subprogram not defined"),
    (37, "Argument-count mismatch"),
    (38, "Array not defined"),
    (40, "Variable required"),
    (50, "FIELD overflow"),
    (51, "Internal error"),
    (52, "Bad file name or number"),
    (53, "File not found"),
    (54, "Bad file mode"),
    (55, "File already open"),
    (56, "FIELD statement active"),
    (57, "Device I/O error"),
    (58, "File already exists"),
    (59, "Bad record length"),
    (61, "Disk full"),
    (62, "Input past end of file"),
    (63, "Bad record number"),
    (64, "Bad file name"),
    (67, "Too many files"),
    (68, "Device unavailable"),
    (69, "Communication-buffer overflow"),
    (70, "Permission denied"),
    (71, "Disk not ready"),
    (72, "Disk-media error"),
    (73, "Feature unavailable"),
    (74, "Rename across disks"),
    (75, "Path/File access error"),
    (76, "Path not found"),
    (258, "Invalid handle"),
    (300, "Memory region out of range"),
    (301, "Invalid size"),
    (302, "Source memory region out of range"),
    (303, "Destination memory region out of range"),
    (304, "Source and destination memory regions out of range"),
    (305, "Source memory has been freed"),
    (306, "Destination memory has been freed"),
    (307, "Memory already freed"),
    (308, "Memory has been freed"),
    (309, "Memory not initialized"),
    (310, "Source memory not initialized"),
    (311, "Destination memory not initialized"),
    (312, "Source and destination memory not initialized"),
    (313, "Source and destination memory have been freed"),
    (314, "_ASSERT failed"),
    (315, "_ASSERT failed (check console for description)"),
];

/// The critical errors, which `error` reports with a text of their own and no handler. Error 11 is not one in this
/// copy of libqb (`QB64PE-DIVERGENCES.md` D-052): it is in [`TEXTS`].
pub const CRITICAL: &[(i64, &str)] = &[
    (256, "Out of stack space"),
    (259, "Cannot find dynamic library file"),
    (260, "Sub/Function does not exist in dynamic library"),
    (261, "Sub/Function does not exist in dynamic library"),
    (270, "_GL command called outside of SUB _GL's scope"),
    (271, "END/SYSTEM called within SUB _GL's scope"),
];

/// The text of error `code`; libqb's own default for a number it has none for.
pub fn text(code: i64) -> &'static str {
    TEXTS
        .iter()
        .chain(CRITICAL)
        .find(|(n, _)| *n == code)
        .map_or("Unprintable error", |(_, t)| t)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every `case n: return "…";` of `error_code_to_text` in the copy of libqb is in [`TEXTS`] with the same
    /// text, and the table has no other.
    #[test]
    fn the_table_is_libqbs() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../libqb/internal/c/libqb/src/error_handle.cpp"
        );
        let source = std::fs::read(path).unwrap();
        let mut found = Vec::new();
        for line in source.split(|&b| b == b'\n') {
            let line = line.trim_ascii();
            let Some(rest) = line.strip_prefix(b"case ") else {
                continue;
            };
            let Some(colon) = rest.iter().position(|&b| b == b':') else {
                continue;
            };
            let Some(text) = rest[colon + 1..]
                .trim_ascii()
                .strip_prefix(b"return \"")
                .and_then(|t| t.strip_suffix(b"\";"))
            else {
                continue;
            };
            let code: i64 = qb64rust_base::show_bytes(&rest[..colon]).parse().unwrap();
            found.push((code, qb64rust_base::show_bytes(text)));
        }
        let table: Vec<(i64, String)> = TEXTS.iter().map(|(n, t)| (*n, (*t).to_string())).collect();
        assert_eq!(found, table);
    }

    #[test]
    fn texts() {
        assert_eq!(text(5), "Illegal function call");
        assert_eq!(text(11), "Division by zero");
        assert_eq!(text(999), "Unprintable error");
    }
}
