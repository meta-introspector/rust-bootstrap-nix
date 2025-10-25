test_book!(
    Nomicon, "src/doc/nomicon", "nomicon", default=false, submodules=["src/doc/nomicon"];
    Reference, "src/doc/reference", "reference", default=false, submodules=["src/doc/reference"];
    RustdocBook, "src/doc/rustdoc", "rustdoc", default=true;
    RustcBook, "src/doc/rustc", "rustc", default=true;
    RustByExample, "src/doc/rust-by-example", "rust-by-example", default=false, submodules=["src/doc/rust-by-example"];
    EmbeddedBook, "src/doc/embedded-book", "embedded-book", default=false, submodules=["src/doc/embedded-book"];
    TheBook, "src/doc/book", "book", default=false, submodules=["src/doc/book"];
    UnstableBook, "src/doc/unstable-book", "unstable-book", default=true;
    EditionGuide, "src/doc/edition-guide", "edition-guide", default=false, submodules=["src/doc/edition-guide"];
);
