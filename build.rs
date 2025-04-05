fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=tests/examples.rs");

    const EXAMPLES_TESTS: &[u8] = include_bytes!("tests/examples.rs");

    let mut examples_docs = Vec::new();

    examples_docs.extend_from_slice(b"## Usage Example\n");
    examples_docs.extend_from_slice(b"\n");
    examples_docs.extend_from_slice(b"```rust\n");
    examples_docs.extend_from_slice(EXAMPLES_TESTS);
    examples_docs.extend_from_slice(b"\n");
    examples_docs.extend_from_slice(b"```");

    let output_dir = std::env::var("OUT_DIR").unwrap();
    std::fs::write(format!("{output_dir}/examples.md"), examples_docs).unwrap();
}
