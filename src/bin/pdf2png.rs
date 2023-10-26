use pandoc::definition::{Inline, Target};
use pandoc::to_json_filter;
use std::io;
use std::path::PathBuf;

fn pdf2png(mut img: Target) -> Target {
    if img.0.ends_with(".pdf") {
        let newp = PathBuf::from(img.0)
            .with_extension("png")
            .to_string_lossy()
            .to_string();
        img.0 = newp;
        // can't find out how to get the actual path for the images,
        // as the AST doesn't have one. Have to do that externally for
        // now (also might wanna use --resource-path flag if in a
        // separate directory)
    }
    img
}

/// units package command into simple LaTeX
fn process(inline: Inline) -> Inline {
    match inline {
        Inline::Image(a1, a2, a3) => Inline::Image(a1, a2, pdf2png(a3)),
        _ => inline,
    }
}

fn main() -> io::Result<()> {
    to_json_filter(&mut process)
}
