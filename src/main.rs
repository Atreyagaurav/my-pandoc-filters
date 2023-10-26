use pandoc::definition::Inline;
use pandoc::to_json_filter;
use std::io;

fn replace_units_parts(unit: String) -> String {
    if unit.contains("\\unit[") {
        let mut parsed = String::new();
        let mut parts = unit.split("\\unit[");
        parsed.push_str(&parts.next().unwrap());
        for part in parts {
            let e = part.find(']').unwrap();
            let us = part.find('{').unwrap() + 1;
            let ue = part.find('}').unwrap();
            parsed.push_str(&format!("{}\\:\\mathrm{{{}}}", &part[..e], &part[us..ue]))
        }
        parsed
    } else {
        unit
    }
}

/// pandoc doesn't seem to handle dcases, so changing it to cases
fn manage_dcases(expr: String) -> String {
    if expr.contains("dcases") {
        let mut parsed = String::new();
        let mut parts = expr.split("dcases");
        parsed.push_str(&parts.next().unwrap());
        parsed.push_str("cases");
        let expr = parts
            .next()
            .unwrap()
            .replace("for", "\\text{for}\\:")
            .replace("otherwise", "\\text{otherwise}\\:")
            .replace("\\(", "")
            .replace("\\)", "");

        parsed.push_str(&expr);
        parsed.push_str("cases");
        parsed.push_str(&parts.next().unwrap());
        parsed
    } else {
        expr
    }
}

/// pandoc doesn't seem to handle align, so changing it to equation
fn manage_align(expr: String) -> String {
    if expr.contains("aligned") {
        let mut parsed = String::new();
        let (before, (expr, after)) = expr
            .split_once("\\begin{aligned}")
            .map(|e| (e.0, e.1.split_once("\\end{aligned}").unwrap()))
            .unwrap();
        parsed.push_str(before);
        parsed.push_str("\\begin{equation}");
        // these replacements are risky, like having a matrix inside
        // the align wil be messed up, I need a way to properly parse
        // LaTeX equation into segments
        let expr = expr
            .split("&=")
            .map(|p| p.trim().trim_end_matches("\\\\"))
            .collect::<Vec<&str>>()
            .join("=");

        parsed.push_str(&expr.trim().trim_end_matches("\\\\"));
        parsed.push_str("\\end{equation}");
        parsed.push_str(after);
        parsed
    } else {
        expr
    }
}

/// units package command into simple LaTeX
fn units(inline: Inline) -> Inline {
    match inline {
        Inline::Math(t, text) => {
            Inline::Math(t, manage_align(manage_dcases(replace_units_parts(text))))
        }
        _ => inline,
    }
}

fn main() -> io::Result<()> {
    to_json_filter(&mut units)
}
