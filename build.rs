use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

include!("src/core/library/settings/theme.rs");

fn main() {
    embed_themes();
}

fn embed_themes() {
    let mut themes = Vec::new();

    for entry in fs::read_dir("res/themes").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let content = fs::read_to_string(&path).unwrap();
            let theme: Theme = toml::from_str(&content).unwrap();
            themes.push(theme);
        }
    }

    let out_path = Path::new("src/core/library/settings/themedata.rs");
    {
        let mut out = File::create(out_path).unwrap();

        writeln!(out, "// This is a generated file. Check `build.rs`").unwrap();
        writeln!(out, "use crate::core::library::settings::theme::Theme;\n").unwrap();
        writeln!(out, "use std::collections::HashMap;").unwrap();
        writeln!(
            out,
            "pub fn get_themes() -> HashMap<&'static str, Theme> {{"
        )
        .unwrap();
        writeln!(out, "    let mut m = HashMap::new();").unwrap();

        for theme in themes.iter_mut() {
            theme.base[0] = u32::from_str_radix(&theme.base00, 16).unwrap();
            theme.base[1] = u32::from_str_radix(&theme.base01, 16).unwrap();
            theme.base[2] = u32::from_str_radix(&theme.base02, 16).unwrap();
            theme.base[3] = u32::from_str_radix(&theme.base03, 16).unwrap();
            theme.base[4] = u32::from_str_radix(&theme.base04, 16).unwrap();
            theme.base[5] = u32::from_str_radix(&theme.base05, 16).unwrap();
            theme.base[6] = u32::from_str_radix(&theme.base06, 16).unwrap();
            theme.base[7] = u32::from_str_radix(&theme.base07, 16).unwrap();
            theme.base[8] = u32::from_str_radix(&theme.base08, 16).unwrap();
            theme.base[9] = u32::from_str_radix(&theme.base09, 16).unwrap();
            theme.base[10] = u32::from_str_radix(&theme.base0A, 16).unwrap();
            theme.base[11] = u32::from_str_radix(&theme.base0B, 16).unwrap();
            theme.base[12] = u32::from_str_radix(&theme.base0C, 16).unwrap();
            theme.base[13] = u32::from_str_radix(&theme.base0D, 16).unwrap();
            theme.base[14] = u32::from_str_radix(&theme.base0E, 16).unwrap();
            theme.base[15] = u32::from_str_radix(&theme.base0F, 16).unwrap();

            writeln!(
                out,
                "    m.insert(
        \"{}\", 
        Theme {{ 
            scheme: \"{}\".to_string(), 
            author: \"{}\".to_string(), 
            base00: \"{}\".to_string(), 
            base01: \"{}\".to_string(), 
            base02: \"{}\".to_string(), 
            base03: \"{}\".to_string(), 
            base04: \"{}\".to_string(), 
            base05: \"{}\".to_string(), 
            base06: \"{}\".to_string(), 
            base07: \"{}\".to_string(), 
            base08: \"{}\".to_string(), 
            base09: \"{}\".to_string(), 
            base0A: \"{}\".to_string(), 
            base0B: \"{}\".to_string(), 
            base0C: \"{}\".to_string(), 
            base0D: \"{}\".to_string(), 
            base0E: \"{}\".to_string(), 
            base0F: \"{}\".to_string(),",
                theme.scheme,
                theme.scheme,
                theme.author,
                theme.base00,
                theme.base01,
                theme.base02,
                theme.base03,
                theme.base04,
                theme.base05,
                theme.base06,
                theme.base07,
                theme.base08,
                theme.base09,
                theme.base0A,
                theme.base0B,
                theme.base0C,
                theme.base0D,
                theme.base0E,
                theme.base0F
            )
            .unwrap();

            writeln!(out, "            base: {:?},", theme.base).unwrap();
            writeln!(out, "        }},\n    );").unwrap();
        }

        writeln!(out, "    m").unwrap();
        writeln!(out, "}}").unwrap();
    }

    std::process::Command::new("rustfmt")
        .arg(out_path)
        .status()
        .expect("Failed to run rustfmt");
}
