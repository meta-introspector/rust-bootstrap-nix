#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum DocTests {
    /// Run normal tests and doc tests (default).
    Yes,
    /// Do not run any doc tests.
    No,
    /// Only run doc tests.
    Only,
}
