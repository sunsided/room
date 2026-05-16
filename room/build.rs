use std::env;
use std::fs;
use std::path::Path;

fn generate_statenum() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let info_h = Path::new(&manifest_dir)
        .join("..")
        .join("vendor")
        .join("doomgeneric")
        .join("info.h");

    println!("cargo:rerun-if-changed={}", info_h.display());

    if !info_h.exists() {
        return;
    }

    let content = fs::read_to_string(&info_h).unwrap();
    let mut in_enum = false;
    let mut entries = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("typedef enum") {
            in_enum = true;
            continue;
        }
        if in_enum {
            if trimmed.starts_with("} statenum_t") {
                break;
            }
            if let Some(name) = trimmed.strip_prefix("S_") {
                let name = format!("S_{}", name.trim_end_matches(','));
                entries.push(name);
            }
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("statenum.rs");

    let mut output = String::new();
    for (i, name) in entries.iter().enumerate() {
        output.push_str(&format!("pub const {}: c_int = {};\n", name, i as i32));
    }

    fs::write(&dest_path, output).unwrap();
}

fn main() {
    generate_statenum();
}
