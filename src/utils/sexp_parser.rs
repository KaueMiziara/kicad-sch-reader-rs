use lexpr::Value;

/// Parses a KiCad schematic file (in S-expression format) and prints all found components.
///
/// # Arguments
/// * `content` - A string containing the KiCad schematic file content.
///
/// # Behavior
/// - On successful parsing, it traverses the S-expression tree to find components.
/// - On failure, prints an error message and exits early.
pub fn parse_sexp(content: &str) {
    let sexpr = match lexpr::from_str(content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse S-expression: {}", e);
            return;
        }
    };

    find_symbols(&sexpr);
}

/// Recursively traverses an S-expression value to find KiCad symbol definitions.
///
/// # Arguments
/// * `value` - The S-expression value to traverse (typically the root of a parsed schematic).
///
/// # Behavior
/// - Matches against `Value::Cons` (lists) and `Value::Vector` (arrays).
/// - When a `(symbol ...)` expression is found, extracts and prints its library identifier.
fn find_symbols(value: &Value) {
    match value {
        Value::Cons(cell) => {
            let car = cell.car();
            let cdr = cell.cdr();

            if let Value::Symbol(s) = car {
                if s.as_ref() == "symbol" {
                    if let Some(lib_id) = extract_lib_id(cdr) {
                        println!("Component: {}", lib_id);
                    }
                }
            }

            find_symbols(car);
            find_symbols(cdr);
        }

        Value::Vector(vec) => {
            for item in vec {
                find_symbols(item);
            }
        }

        _ => {}
    }
}

/// Extracts the library identifier (`lib_id`) from a symbol's S-expression.
///
/// # Arguments
/// * `cdr` - The tail of a `(symbol ...)` list containing the symbol's properties.
///
/// # Returns
/// `Some(String)` containing the library ID if found, otherwise `None`.
///
/// # Example
/// For `(symbol (lib_id "Device:R") ...)`, returns `Some("Device:R")`.
fn extract_lib_id(cdr: &Value) -> Option<String> {
    match cdr {
        Value::Cons(cell) => {
            let car = cell.car();
            if let Value::Cons(first_entry) = car {
                let first_key = first_entry.car();
                if let Value::Symbol(s) = first_key {
                    if s.as_ref() == "lib_id" {
                        let lib_val = first_entry.cdr();
                        if let Value::Cons(lib_pair) = lib_val {
                            match lib_pair.car() {
                                Value::Symbol(lib_str) => return Some(lib_str.to_string()),
                                Value::String(lib_str) => return Some(lib_str.to_string()),
                                _ => {}
                            }
                        }
                    }
                }
            }

            extract_lib_id(cell.cdr())
        }

        _ => None,
    }
}
