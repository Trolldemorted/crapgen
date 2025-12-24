use std::path::PathBuf;

pub mod windows;

#[derive(Debug)]
pub(crate) struct ExportedFunction {
    pub ordinal: u64,
    pub name: String,
}

fn main() {
    let input = std::env::args().nth(1).expect("Missing input path");
    let output = std::env::args().nth(2).expect("Missing output path");
    windows::generate(&input, &PathBuf::from(output));
}
